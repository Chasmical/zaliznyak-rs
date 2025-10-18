use crate::{
    categories::{Case, DeclInfo, Gender, IntoNumber},
    declension::{NounDeclension, NounStemType},
    stress::NounStress,
    util::InflectionBuf,
    word::Utf8Letter,
};

type Pos = u8;

enum VowelAlternation {
    A(Pos, Option<A_Replace>),
    B(B_Operation),
}

#[allow(non_camel_case_types)]
enum A_Replace {
    Й,
    Ь,
}
#[allow(non_camel_case_types)]
enum B_Operation {
    ReplaceLastWithI,
    ReplacePreLastWithYe,
    ReplacePreLastWithYeOrYo,
    InsertYe,
    InsertO,
    InsertYeOrYo,
    InsertYeOrO,
}

impl VowelAlternation {
    pub fn prepare_noun(stem: &[Utf8Letter], decl: NounDeclension, gender: Gender) -> Option<Self> {
        if gender == Gender::Masculine
            || gender == Gender::Feminine && decl.stem_type == NounStemType::Type8
        {
            // Vowel alternation type A (masc any / fem 8*)
            // Affects all forms, except sg nom / sg fem ins

            // Find the alternating LAST vowel in stem
            let Some(found) = stem.iter().enumerate().rfind(|x| x.1.is_vowel()) else {
                todo!("Handle absence of vowels in the stem?")
            };
            let (vowel_pos, vowel) = found;

            let replace = match vowel {
                // 'о' is removed from the word
                Utf8Letter::О => None,

                Utf8Letter::Е | Utf8Letter::Ё => {
                    let preceding = stem.get(vowel_pos - 1).copied();

                    #[allow(unused_parens)]
                    if preceding.is_some_and(|x| x.is_vowel()) {
                        // 1) is replaced with 'й' when after a vowel
                        Some(A_Replace::Й)
                    } else if (
                        // 2)a) is replaced with 'ь', if masc 6*
                        decl.stem_type == NounStemType::Type6
                        // 2)b) is replaced with 'ь', if masc 3* and after non-sibilant consonant
                        || decl.stem_type == NounStemType::Type3
                            && preceding.is_some_and(|x| x.is_non_sibilant_consonant())
                        // 2)c) is replaced with 'ь', when after 'л'
                        || preceding == Some(Utf8Letter::Л)
                    ) {
                        Some(A_Replace::Ь)
                    } else {
                        // 3) is removed in all other cases
                        None
                    }
                },
                _ => {
                    todo!("Handle invalid vowel alternation")
                },
            };

            return Some(Self::A(vowel_pos as _, replace));
        }

        if matches!(gender, Gender::Neuter | Gender::Feminine) {
            // Vowel alternation type B (neuter any / fem 1-7*)
            // Affects only plural genitive forms

            // TODO: 2*b and 2*f are exempt from vowel alternation for some reason?
            // E.g. песня (ж 2*a) - Р.мн. песен; лыжня (ж 2*b) - Р.мн. лыжней, not лыжен.
            if decl.stem_type == NounStemType::Type2
                && matches!(decl.stress, NounStress::B | NounStress::F)
            {
                return None;
            }
            // If (2) flag changed the ending's gender, don't alternate the vowel,
            //   since it won't be consistent with the ending of different gender.
            if decl.flags.has_circled_two() {
                return None;
            }

            // 1) stem type 6: stem's ending 'ь' is replaced with 'е' or 'и'.
            // E.g. лгунья (ж 6*a) - Р.мн. лгуний; статья (ж 6*b) - Р.мн. статей.
            if decl.stem_type == NounStemType::Type6 {
                if let [.., Utf8Letter::Ь] = stem {
                    return Some(Self::B(B_Operation::ReplaceLastWithI));
                }
                // Alternations in stem type 6 happen only with 'ь'.
                return None;
            }

            // At this point, stem type is in range 1..=5 (consonant-ending stems).
            // Stem type 6 was completely handled earlier, and 7* nouns don't exist.
            // So, it's safe to assume that the last stem char is a consonant.
            let last = stem.last().copied();
            let pre_last = stem.get(stem.len() - 2).copied();

            // 2) if 'ь'/'й' precedes the last consonant, replace 'ь'/'й' with 'ё' or 'е'.
            // E.g. гайка (ж 3*a) - Р.мн. гаек; сальце (с 5*a) - Р.мн. салец.
            if let Some(Utf8Letter::Ь | Utf8Letter::Й) = pre_last {
                if last == Some(Utf8Letter::Ц) {
                    return Some(Self::B(B_Operation::ReplacePreLastWithYe));
                } else {
                    return Some(Self::B(B_Operation::ReplacePreLastWithYeOrYo));
                };
            }

            // 3) in all other cases, insert a letter between two last chars

            if let Some(Utf8Letter::К | Utf8Letter::Г | Utf8Letter::Х) = pre_last {
                // 3)a) after 'к'/'г'/'х' insert 'о'
                return Some(Self::B(B_Operation::InsertO));
            }
            if let Some(Utf8Letter::К | Utf8Letter::Г | Utf8Letter::Х) = last
                && pre_last.is_some_and(|x| !x.is_sibilant())
            {
                // 3)b) before 'к'/'г'/'х', but not after sibilant, insert 'о'
                return Some(Self::B(B_Operation::InsertO));
            }

            // 3)c) if unstressed insert 'е', and if stressed - 'ё'

            if last == Some(Utf8Letter::Ц) {
                // But after 'ц' only 'е'
                return Some(Self::B(B_Operation::InsertYe));
            }
            if pre_last.is_some_and(|x| x.is_hissing()) {
                // And after hissing consonants 'о' instead of 'ё'
                return Some(Self::B(B_Operation::InsertYeOrO));
            }
            return Some(Self::B(B_Operation::InsertYeOrYo));
        }

        todo!("Invalid vowel alternation")
    }

    pub fn apply(&self, decl: NounDeclension, info: DeclInfo, buf: &mut InflectionBuf) {
        match self {
            Self::A(vowel_pos, replace) => {
                // Affects all forms, except sg nom / sg fem ins
                if info.is_singular() {
                    if info.case.is_nom_or_acc_inan(info) {
                        return;
                    }
                    if info.gender == Gender::Feminine && info.case == Case::Instrumental {
                        return;
                    }
                }

                if let Some(replace) = replace {
                    let vowel =
                        unsafe { buf.as_mut_slice().get_unchecked_mut(*vowel_pos as usize) };
                    *vowel = match replace {
                        A_Replace::Й => Utf8Letter::Й,
                        A_Replace::Ь => Utf8Letter::Ь,
                    };
                } else {
                    buf.remove_stem_char_at(*vowel_pos as usize);
                }
            },

            Self::B(operation) => {
                // Affects only plural genitive forms
                if !(info.is_plural() && info.case.is_gen_or_acc_an(info)) {
                    return;
                }

                // Special case for feminine 2*a, ending with 'ня': remove 'ь' ending.
                // E.g. вафля (2*a) - Р.мн. вафель; башня (2*a) - Р.мн. башен, not башень.
                // Note: only stem type 2*a nouns can have 'ь' as ending here.
                // (see declension::endings_tables::NOUN_LOOKUP, 'gen pl' section)
                if buf.is_stem_stressed() && decl.stem_type == NounStemType::Type2 {
                    buf.replace_ending("");
                }

                let insert = match operation {
                    B_Operation::ReplaceLastWithI => {
                        let stressed = buf.is_ending_stressed();
                        let letter = if stressed { Utf8Letter::Е } else { Utf8Letter::И };

                        let last = unsafe { buf.stem_mut().last_mut().unwrap_unchecked() };
                        *last = letter;
                        return;
                    },
                    B_Operation::ReplacePreLastWithYe => {
                        let pre_last = unsafe {
                            &mut buf.stem_mut().last_chunk_mut::<2>().unwrap_unchecked()[0]
                        };
                        *pre_last = Utf8Letter::Е;
                        return;
                    },
                    B_Operation::ReplacePreLastWithYeOrYo => {
                        let stressed = buf.is_ending_stressed();
                        let letter = if stressed { Utf8Letter::Ё } else { Utf8Letter::Е };

                        let pre_last = unsafe {
                            &mut buf.stem_mut().last_chunk_mut::<2>().unwrap_unchecked()[0]
                        };
                        *pre_last = letter;
                        return;
                    },

                    B_Operation::InsertO => Utf8Letter::О,
                    B_Operation::InsertYe => Utf8Letter::Е,
                    B_Operation::InsertYeOrO if buf.is_ending_stressed() => Utf8Letter::О,
                    B_Operation::InsertYeOrYo if buf.is_ending_stressed() => Utf8Letter::Ё,
                    _ => Utf8Letter::Е,
                };

                buf.insert_between_last_two_stem_chars(insert.as_str());
            },
        }
    }
}

impl NounDeclension {
    pub(crate) fn apply_vowel_alternation(&self, info: DeclInfo, buf: &mut InflectionBuf) {
        VowelAlternation::prepare_noun(buf.stem(), *self, info.gender)
            .inspect(|x| x.apply(*self, info, buf));
    }
}
