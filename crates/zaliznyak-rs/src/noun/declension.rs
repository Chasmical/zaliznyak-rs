use crate::{
    alphabet::Utf8Letter,
    categories::{Case, DeclInfo, Gender, IntoNumber},
    declension::{NounDeclension, NounStemType},
    inflection_buf::InflectionBuf,
    stress::NounStress,
};

impl NounDeclension {
    pub fn inflect(self, info: DeclInfo, buf: &mut InflectionBuf) {
        buf.append_to_ending(self.find_ending(info));

        if self.flags.has_circle() {
            self.apply_unique_alternation(info, buf);
        }

        // Special case for stem type 8: endings from the table may start with 'я', while
        // the stem ends with a hissing consonant. Replace 'я' with 'а' in this case.
        // E.g. мышь (жо 8e) - Д.мн. мышАм, not мышЯм.
        if self.stem_type == NounStemType::Type8
            && buf.stem().last().is_some_and(|x| x.is_hissing())
            && let [ya @ Utf8Letter::Я, ..] = buf.ending_mut()
        {
            *ya = Utf8Letter::А;
        }

        if self.flags.has_star() {
            self.apply_vowel_alternation(info, buf);
        }

        // The е/ё alternation is handled more efficiently in apply_unique_alternation()
        if self.flags.has_alternating_yo() && !self.flags.has_circle() {
            self.apply_ye_yo_alternation(info, buf);
        }
    }

    pub fn apply_unique_alternation(self, info: DeclInfo, buf: &mut InflectionBuf) {
        use Utf8Letter::*;

        match buf.stem_mut() {
            // -ин (боярин, крестьянин, землянин, господин)
            [.., И, Н] => {
                if info.is_plural() {
                    // Shrink by 2 chars, removing 'ин'
                    buf.shrink_stem_by(2);

                    // Nominative - ending 'е', Genitive - ending '', other - no changes
                    if let Some(is_gen) = info.case.acc_is_gen(info) {
                        buf.replace_ending(if is_gen {
                            ""
                        } else {
                            // Don't override if (1) flag already did (господин - господа)
                            if self.flags.has_circled_one() { return } else { "е" }
                        });
                    }
                }
            },
            // -[оё]нок (утёнок, ребёнок, опёнок, мышонок, зайчонок)
            [.., yo @ (О | Ё), n @ Н, О, К] => {
                if info.is_plural() {
                    // Transform '-[оё]нок' into '-[ая]т'

                    // Replace 'о' with 'а', and 'ё' with 'я'
                    *yo = if *yo == О { А } else { Я };
                    // Set the stem char after '[ая]' to 'т'
                    *n = Т;
                    // Shrink by 2 chars, removing 'ок'
                    buf.shrink_stem_by(2);

                    // Nominative - ending 'а', genitive - ending '', other - no changes
                    if let Some(is_gen) = info.case.acc_is_gen(info) {
                        buf.replace_ending(if is_gen { "" } else { "а" });
                    }
                } else {
                    // Remove pre-last char ('о') in non-nominative forms
                    if !info.case.is_nom_or_acc_inan(info) {
                        buf.remove_pre_last_stem_char();
                    }
                }
            },
            // -ок (щенок, внучок)
            [.., preceding, o @ О, k @ К] => {
                if info.is_plural() {
                    // Transform '-ок' into '-[ая]т'

                    // If preceded by a sibilant, replace 'о' with 'а'; otherwise, with 'я'
                    *o = if preceding.is_sibilant() { А } else { Я };
                    // Set the stem char after '[ая]' to 'т'
                    *k = Т;

                    // Nominative - ending 'а', genitive - ending '', other - no changes
                    if let Some(is_gen) = info.case.acc_is_gen(info) {
                        buf.replace_ending(if is_gen { "" } else { "а" });
                    }
                } else {
                    // Remove pre-last char ('о') in non-nominative forms
                    if !info.case.is_nom_or_acc_inan(info) {
                        buf.remove_pre_last_stem_char();
                    }
                }
            },
            // -[оё]ночек (телёночек, котёночек, мышоночек)
            [.., yo @ (О | Ё), n @ Н, o @ О, Ч, Е, К] => {
                if info.is_plural() {
                    // Transform '-[оё]ночек' into '-[ая]тк'

                    // Replace 'о' with 'а', and 'ё' with 'я'
                    *yo = if *yo == О { А } else { Я };
                    // Set the stem chars after '[ая]' to 'тк'
                    (*n, *o) = (Т, К);
                    // Shrink by 3 chars, removing 'чек'
                    buf.shrink_stem_by(3);

                    // Genitive - insert 'о' between 'т' and 'к'
                    if info.case.is_gen_or_acc_an(info) {
                        buf.insert_between_last_two_stem_chars("о");
                        buf.replace_ending("");
                    }
                } else {
                    // Remove pre-last char ('е') in non-nominative forms
                    if !info.case.is_nom_or_acc_inan(info) {
                        buf.remove_pre_last_stem_char();
                    }
                }
            },
            // -очек (щеночек, внучоночек)
            [.., preceding, o @ О, ch @ Ч, ie @ Е, К] => {
                if info.is_plural() {
                    // Transform '-очек' into '-[ая]тк'

                    // If preceded by a sibilant, replace 'о' with 'а'; otherwise, with 'я'
                    *o = if preceding.is_sibilant() { А } else { Я };
                    // Set the stem chars after '[ая]' to 'тк'
                    (*ch, *ie) = (Т, К);
                    // Shrink by 1 char, removing 'к'
                    buf.shrink_stem_by(1);

                    // Genitive - insert 'о' between 'т' and 'к'
                    if info.case.is_gen_or_acc_an(info) {
                        buf.insert_between_last_two_stem_chars("о");
                        buf.replace_ending("");
                    }
                } else {
                    // Remove pre-last char ('е') in non-nominative forms
                    if !info.case.is_nom_or_acc_inan(info) {
                        buf.remove_pre_last_stem_char();
                    }
                }
            },
            // -мя (время, знамя, пламя, имя)
            [.., М] if info.gender == Gender::Neuter => {
                if info.is_plural() {
                    // The е/ё is handled here, instead of in apply_ye_yo_alternation()
                    let yo = self.flags.has_alternating_yo() && info.case.is_gen_or_acc_an(info);
                    // Add '[её]н' suffix to the stem
                    buf.append_to_stem(if yo { "ён" } else { "ен" });
                } else {
                    // Replace nominative singular ending 'ь' with 'я'
                    if let [ending @ Ь] = buf.ending_mut() {
                        *ending = Я;
                    } else {
                        // In non-nominative forms, add 'ен' suffix to the stem
                        buf.append_to_stem("ен");
                    }
                }
            },
            _ => unimplemented!(),
        };
    }

    pub fn apply_vowel_alternation(self, info: DeclInfo, buf: &mut InflectionBuf) {
        use Utf8Letter::*;

        // Extend stem's lifetime, to allow accessing ending() and then setting stem chars
        let stem = unsafe { std::mem::transmute::<_, &mut [Utf8Letter]>(buf.stem_mut()) };

        if info.gender == Gender::Masculine
            || info.gender == Gender::Feminine && self.stem_type == NounStemType::Type8
        {
            // Vowel alternation type A (masc any / fem 8*)

            if info.is_singular() {
                // Singular nominative form is unchanged (and accusative inanimate too)
                if info.case.is_nom_or_acc_inan(info) {
                    return;
                }
                // Singular instrumental for feminine 8* is unchanged (ending with 'ью')
                if info.gender == Gender::Feminine && info.case == Case::Instrumental {
                    return;
                }
            }

            // Find the alternating LAST vowel
            let Some(found) = stem.iter_mut().enumerate().rfind(|x| x.1.is_vowel()) else {
                todo!("Handle absence of vowels in the stem?")
            };
            let (vowel_index, vowel) = found;

            // Extend vowel's lifetime, to allow accessing stem() and then setting vowel
            let vowel = unsafe { std::mem::transmute::<&mut Utf8Letter, &mut Utf8Letter>(vowel) };

            match vowel {
                О => {
                    // 'о' is simply removed
                    buf.remove_stem_char_at(vowel_index);
                },
                Е | Ё => {
                    let preceding = stem.get(vowel_index - 1).copied();

                    #[rustfmt::skip] #[allow(unused_parens)]
                    if preceding.is_some_and(|x| x.is_vowel()) {
                        // 1) is replaced with 'й' when after a vowel
                        *vowel = Й;
                    } else if (
                        // 2)a) is replaced with 'ь', if masc 6*
                        self.stem_type == NounStemType::Type6
                        // 2)b) is replaced with 'ь', if masc 3* and after non-sibilant consonant
                        || self.stem_type == NounStemType::Type3
                            && preceding.is_some_and(|x| x.is_non_sibilant_consonant())
                        // 2)c) is replaced with 'ь', when after 'л'
                        || preceding == Some(Л)
                    )
                    {
                        *vowel = Ь;
                    };
                },
                _ => {
                    todo!("Handle invalid vowel alternation")
                },
            };
            return;
        }
        if matches!(info.gender, Gender::Neuter | Gender::Feminine)
            && info.is_plural()
            && info.case.is_gen_or_acc_an(info)
        {
            // Vowel alternation type B (neuter any / fem 1-7*)
            // Affects only plural genitive forms

            // TODO: 2*b and 2*f are exempt from vowel alternation for some reason?
            // E.g. песня (ж 2*a) - Р.мн. песен; лыжня (ж 2*b) - Р.мн. лыжней, not лыжен.
            if self.stem_type == NounStemType::Type2
                && matches!(self.stress, NounStress::B | NounStress::F)
            {
                return;
            }
            // If (2) flag changed the ending's gender, don't alternate the vowel,
            // since it won't be consistent with the ending of different gender.
            if self.flags.has_circled_two() {
                return;
            }

            // 1) stem type 6: stem's ending 'ь' is replaced with 'е' or 'и'.
            // E.g. лгунья (ж 6*a) - Р.мн. лгуний; статья (ж 6*b) - Р.мн. статей.
            if self.stem_type == NounStemType::Type6 {
                if let [.., last @ Ь] = stem {
                    let ending_stressed = self.stress.is_ending_stressed(info);
                    *last = if ending_stressed { Е } else { И };
                }
                // Alternations in stem type 6 happen only with 'ь'.
                return;
            }

            // Special case for feminine 2*a, ending with 'ня': remove 'ь' ending.
            // E.g. вафля (2*a) - Р.мн. вафель; башня (2*a) - Р.мн. башен, not башень.
            // Note: only stem type 2*a nouns can have 'ь' as ending here.
            // (see declension::endings_tables::NOUN_LOOKUP, 'gen pl' section)
            if let [Ь] = buf.ending()
                && let [.., Н] = stem
            {
                buf.replace_ending("");
            }

            // At this point, stem type is in range 1..=5 (consonant-ending stems).
            // Stem type 6 was completely handled earlier, and 7* nouns don't exist.
            // So, it's safe to assume that the last stem char is a consonant.
            let last = stem.last().copied();
            let pre_last = stem.get_mut(stem.len() - 2);

            // 2) if 'ь'/'й' precedes the last consonant, replace 'ь'/'й' with 'ё' or 'е'.
            // E.g. гайка (ж 3*a) - Р.мн. гаек; сальце (с 5*a) - Р.мн. салец.
            if let Some(pre_last @ (Ь | Й)) = pre_last {
                let stressed = last != Some(Ц) && self.stress.is_ending_stressed(info);
                *pre_last = if stressed { Ё } else { Е };
                return;
            }

            // 3) in all other cases, insert a letter between two last chars
            let insert_between = {
                // 3)a) after 'к'/'г'/'х' insert 'о'
                if let Some(К | Г | Х) = pre_last {
                    О
                }
                // 3)b) before 'к'/'г'/'х', but not after sibilant, insert 'о'
                else if let Some(К | Г | Х) = last
                    && let Some(ref pre_last) = pre_last
                    && pre_last.is_sibilant()
                {
                    О
                }
                // 3)c) if unstressed insert 'е', and if stressed - 'е'
                else {
                    // But after 'ц' always insert 'е'
                    if last == Some(Ц) || self.stress.is_stem_stressed(info) {
                        Е
                    } else {
                        // And after hissing consonants insert 'о' instead of 'ё'
                        if pre_last.is_some_and(|x| x.is_hissing()) { О } else { Ё }
                    }
                }
            };
            buf.insert_between_last_two_stem_chars(insert_between.as_str());
        }
    }

    pub fn apply_ye_yo_alternation(self, info: DeclInfo, buf: &mut InflectionBuf) {
        let (stem, ending) = buf.stem_and_ending_mut();

        // If there's a 'ё' in the stem:
        if let Some(yo) = stem.iter_mut().find(|x| **x == Utf8Letter::Ё) {
            // If stress falls on the ending, unstress 'ё' in the stem into 'е'
            if self.stress.is_ending_stressed(info) && ending.iter().any(|x| x.is_vowel()) {
                *yo = Utf8Letter::Е;
            }
        } else {
            // If there's no 'ё' in the stem, find the 'е' that can be stressed into 'ё'
            let mut search_stem = stem;

            // If there was vowel alternation, exclude the last two letters from the search,
            // since a 'е' may have been inserted in there, that shouldn't be turned into 'ё'.
            // E.g. метла (ж 1*d, ё) - Р.мн. мЁтел, not метЁл.
            // TODO: See if the е/ё can be put before vowel alternation to avoid this workaround.
            if self.flags.has_star()
                && let [new_search_stem @ .., _, _] = search_stem
            {
                search_stem = new_search_stem;
            }

            // Find the LAST unstressed 'е' in the stem
            let Some(ye) = search_stem.iter_mut().rfind(|x| **x == Utf8Letter::Е) else {
                todo!("Handle absence of 'е' in the stem?")
            };
            // Extend ye's lifetime, to allow accessing stem() and then setting ye
            let ye = unsafe { std::mem::transmute::<&mut Utf8Letter, &mut Utf8Letter>(ye) };

            let stress_into_yo = {
                if !ending.iter().any(|x| x.is_vowel()) {
                    // If the ending can't receive stress, then stress 'е' in the stem into 'ё'
                    true
                } else {
                    if matches!(self.stress, NounStress::F | NounStress::Fp | NounStress::Fpp) {
                        // Special case for f/f′/f″ stress nouns: the last 'е' in the stem
                        // can receive stress only if it's the only vowel in the stem.
                        // E.g. железа (1f, ё) - И.мн. железы; середа (1f′, ё) - В.ед. середу;
                        //       слеза (1f, ё) - И.мн. слёзы;    щека (3f′, ё) - В.ед. щёку.
                        let first_vowel = buf.stem().iter().find(|x| x.is_vowel());

                        first_vowel.is_some_and(|x| std::ptr::eq(ye, x))
                    } else {
                        // In all other cases, stress 'е' in the stem into 'ё'
                        true
                    }
                }
            };

            // Stress 'е' in the stem into 'ё'
            if stress_into_yo {
                *ye = Utf8Letter::Ё;
            }
        }
    }
}
