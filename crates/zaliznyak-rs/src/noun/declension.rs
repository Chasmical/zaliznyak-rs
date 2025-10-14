use crate::{
    categories::{Case, CaseEx, DeclInfo, Gender, IntoNumber, Number},
    declension::{Declension, NounDeclension, NounStemType},
    noun::{Noun, NounInfo},
    stress::NounStress,
    util::InflectionBuf,
    word::{Utf8Letter, Utf8LetterSlice, Word, WordBuf},
};

impl Noun {
    pub fn inflect(&self, case: CaseEx, number: Number) -> WordBuf {
        self.info.inflect(self.stem.borrow(), case, number)
    }

    pub fn inflect_into<'a>(
        &self,
        case: CaseEx,
        number: Number,
        dst: &'a mut [Utf8Letter],
    ) -> Word<'a> {
        self.info.inflect_into(self.stem.borrow(), case, number, dst)
    }
}

impl NounInfo {
    pub fn inflect(&self, stem: Word, case: CaseEx, number: Number) -> WordBuf {
        let mut buf = WordBuf::with_capacity_for(stem);
        buf.inflect(|dst| self.inflect_into(stem, case, number, dst));
        buf
    }

    pub fn inflect_into<'a>(
        &self,
        stem: Word,
        case: CaseEx,
        number: Number,
        dst: &'a mut [Utf8Letter],
    ) -> Word<'a> {
        let mut buf = InflectionBuf::with_stem_in(stem.as_letters(), dst);

        if let Some(decl) = self.declension {
            let number = self.tantum.unwrap_or(number);
            let (case, number) = case.normalize_with(number);

            let info =
                DeclInfo { case, number, gender: self.declension_gender, animacy: self.animacy };

            match decl {
                Declension::Noun(decl) => decl.inflect(info, &mut buf),
                Declension::Adjective(decl) => decl.inflect(info, &mut buf),
                Declension::Pronoun(_) => unimplemented!(), // Nouns don't decline by pronoun declension
            };
        }

        buf.into()
    }
}

impl NounDeclension {
    pub(crate) fn inflect(self, info: DeclInfo, buf: &mut InflectionBuf) {
        buf.append_to_ending(self.find_ending(info).as_str());

        if self.flags.has_circle() {
            self.apply_unique_alternation(info, buf);
        }

        // Special case for stem type 8: endings from the table may start with 'я', while
        //   the stem ends with a hissing consonant. Replace 'я' with 'а' in this case.
        // E.g. мышь (жо 8e) - Д.мн. мышАм, not мышЯм.
        if self.stem_type == NounStemType::Type8
            && buf.stem().last().is_some_and(|x| x.is_hissing())
            && let [ya @ Utf8Letter::Я, ..] = buf.ending_mut()
        {
            *ya = Utf8Letter::А;
        }

        // The е/ё alternation is handled more efficiently in apply_unique_alternation()
        if self.flags.has_alternating_yo() && !self.flags.has_circle() {
            self.apply_ye_yo_alternation(info, buf);
        }

        if self.flags.has_star() {
            self.apply_vowel_alternation(info, buf);
        }
    }

    fn apply_unique_alternation(self, info: DeclInfo, buf: &mut InflectionBuf) {
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
            // -очек (щеночек, внучочек)
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
                    // In singular forms just replace the ending

                    if info.case.is_nom_or_acc_inan(info) {
                        // Replace nominative singular ending 'ь'/'о' with 'я'
                        buf.replace_ending("я");
                    } else {
                        // In non-nominative forms, add 'ен' suffix to the stem
                        buf.append_to_stem("ен");
                        // Instrumental case - ending 'ем', other cases - 'и'
                        let is_ins = info.case == Case::Instrumental;
                        buf.replace_ending(if is_ins { "ем" } else { "и" });
                    }
                }
            },
            _ => unimplemented!(),
        };
    }

    fn apply_vowel_alternation(self, info: DeclInfo, buf: &mut InflectionBuf) {
        use Utf8Letter::*;

        if info.gender == Gender::Masculine
            || info.gender == Gender::Feminine && self.stem_type == NounStemType::Type8
        {
            let stem = buf.stem_mut();

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

            // SAFETY: The InflectionBuf isn't modified between here and the assignment of vowel.
            let vowel = unsafe { &mut *&raw mut *vowel };

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
                    ) {
                        *vowel = Ь;
                    } else {
                        // 3) removed in all other cases
                        buf.remove_stem_char_at(vowel_index);
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
            //   since it won't be consistent with the ending of different gender.
            if self.flags.has_circled_two() {
                return;
            }

            // 1) stem type 6: stem's ending 'ь' is replaced with 'е' or 'и'.
            // E.g. лгунья (ж 6*a) - Р.мн. лгуний; статья (ж 6*b) - Р.мн. статей.
            if self.stem_type == NounStemType::Type6 {
                if let [.., last @ Ь] = buf.stem_mut() {
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
            if buf.ending().len() == 1
                && let [.., Н, Ь] = buf.as_slice()
            {
                buf.replace_ending("");
            }

            // At this point, stem type is in range 1..=5 (consonant-ending stems).
            // Stem type 6 was completely handled earlier, and 7* nouns don't exist.
            // So, it's safe to assume that the last stem char is a consonant.
            let stem = buf.stem_mut();
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
                    && let Some(pre_last) = &pre_last
                    && !pre_last.is_sibilant()
                {
                    О
                }
                // 3)c) if unstressed insert 'е', and if stressed - 'ё'
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

    fn apply_ye_yo_alternation(self, info: DeclInfo, buf: &mut InflectionBuf) {
        let (stem, ending) = buf.stem_and_ending_mut();

        // If there's a 'ё' in the stem:
        if let Some(yo) = stem.iter_mut().find(|x| **x == Utf8Letter::Ё) {
            // If stress falls on the ending, unstress 'ё' in the stem into 'е'
            if self.stress.is_ending_stressed(info) && ending.iter().any(|x| x.is_vowel()) {
                *yo = Utf8Letter::Е;
            }
        } else {
            // If there's no 'ё' in the stem, find the 'е' that can be stressed into 'ё'

            // Find the LAST unstressed 'е' in the stem
            let Some(ye) = stem.iter_mut().rfind(|x| **x == Utf8Letter::Е) else {
                todo!("Handle absence of 'е' in the stem?")
            };
            // SAFETY: The InflectionBuf isn't modified between here and the assignment of ye.
            let ye = unsafe { &mut *&raw mut *ye };

            let stress_into_yo = {
                if ending.iter().any(|x| x.is_vowel()) {
                    // If ending has a vowel, see if it receives stress or not

                    if matches!(self.stress, NounStress::F | NounStress::Fp | NounStress::Fpp) {
                        // Special case for f/f′/f″ stress nouns: the last 'е' in the stem
                        //   can receive stress only if it's the only vowel in the stem.
                        // E.g. железа (1f, ё) - И.мн. железы; середа (1f′, ё) - В.ед. середу;
                        //       слеза (1f, ё) - И.мн. слёзы;    щека (3f′, ё) - В.ед. щёку.
                        let first_vowel = stem.iter().find(|x| x.is_vowel());

                        first_vowel.is_some_and(|x| std::ptr::eq(ye, x))
                            && self.stress.is_stem_stressed(info)
                    } else {
                        // In all other cases, stress 'е' in the stem into 'ё'
                        self.stress.is_stem_stressed(info)
                    }
                } else {
                    // No vowels in ending, stress 'е' in the stem into 'ё'
                    true
                }
            };

            // Stress 'е' in the stem into 'ё'
            if stress_into_yo {
                *ye = Utf8Letter::Ё;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decl(word: &str, info: &str) -> [String; 2] {
        let word: WordBuf = word.parse().unwrap();

        let stem = NounStemType::identify(word.as_letters()).unwrap().0;
        let stem = Word::new(stem, stem.len(), 0).to_owned();

        let noun = Noun { stem, info: info.parse().unwrap() };

        Number::VALUES.map(|number| {
            Case::VALUES.map(|case| noun.inflect(case.into(), number).into_string()).join(", ")
        })
    }

    #[test]
    fn simple_stem_types() {
        // Simple stem type 1
        assert_eq!(decl("топо́р", "м 1b"), [
            "топо́р, топора́, топору́, топо́р, топоро́м, топоре́",
            "топоры́, топоро́в, топора́м, топоры́, топора́ми, топора́х",
        ]);
        assert_eq!(decl("кобра", "жо 1a"), [
            "кобра, кобры, кобре, кобру, коброй, кобре",
            "кобры, кобр, кобрам, кобр, кобрами, кобрах",
        ]);
        assert_eq!(decl("олово", "с 1a"), [
            "олово, олова, олову, олово, оловом, олове",
            "олова, олов, оловам, олова, оловами, оловах",
        ]);

        // Simple stem type 2
        assert_eq!(decl("искатель", "мо 2a"), [
            "искатель, искателя, искателю, искателя, искателем, искателе",
            "искатели, искателей, искателям, искателей, искателями, искателях",
        ]);
        assert_eq!(decl("баня", "ж 2a"), [
            "баня, бани, бане, баню, баней, бане",
            "бани, бань, баням, бани, банями, банях",
        ]);
        assert_eq!(decl("поле", "с 2c"), [
            "поле, поля, полю, поле, полем, поле",
            "поля, полей, полям, поля, полями, полях",
        ]);

        // Simple stem type 3
        assert_eq!(decl("блинчик", "мо 3a"), [
            "блинчик, блинчика, блинчику, блинчика, блинчиком, блинчике",
            "блинчики, блинчиков, блинчикам, блинчиков, блинчиками, блинчиках",
        ]);
        assert_eq!(decl("коряга", "ж 3a"), [
            "коряга, коряги, коряге, корягу, корягой, коряге",
            "коряги, коряг, корягам, коряги, корягами, корягах",
        ]);
        assert_eq!(decl("войско", "с 3c"), [
            "войско, войска, войску, войско, войском, войске",
            "войска, войск, войскам, войска, войсками, войсках",
        ]);

        // Simple stem type 4
        assert_eq!(decl("калач", "м 4b"), [
            "калач, калача, калачу, калач, калачом, калаче",
            "калачи, калачей, калачам, калачи, калачами, калачах",
        ]);
        assert_eq!(decl("галоша", "ж 4a"), [
            "галоша, галоши, галоше, галошу, галошей, галоше",
            "галоши, галош, галошам, галоши, галошами, галошах",
        ]);
        assert_eq!(decl("жилище", "с 4a"), [
            "жилище, жилища, жилищу, жилище, жилищем, жилище",
            "жилища, жилищ, жилищам, жилища, жилищами, жилищах",
        ]);

        // Simple stem type 5
        assert_eq!(decl("кузнец", "мо 5b"), [
            "кузнец, кузнеца, кузнецу, кузнеца, кузнецом, кузнеце",
            "кузнецы, кузнецов, кузнецам, кузнецов, кузнецами, кузнецах",
        ]);
        assert_eq!(decl("девица", "жо 5a"), [
            "девица, девицы, девице, девицу, девицей, девице",
            "девицы, девиц, девицам, девиц, девицами, девицах",
        ]);
        assert_eq!(decl("лицо", "с 5d"), [
            "лицо, лица, лицу, лицо, лицом, лице",
            "лица, лиц, лицам, лица, лицами, лицах",
        ]);

        // Simple stem type 6
        assert_eq!(decl("бой", "м 6c"), [
            "бой, боя, бою, бой, боем, бое",
            "бои, боёв, боям, бои, боями, боях",
        ]);
        assert_eq!(decl("шея", "ж 6a"), [
            "шея, шеи, шее, шею, шеей, шее",
            "шеи, шей, шеям, шеи, шеями, шеях",
        ]);
        // All stem type 6 neuters have vowel alternations

        // Simple stem type 7
        assert_eq!(decl("полоний", "м 7a"), [
            "полоний, полония, полонию, полоний, полонием, полонии",
            "полонии, полониев, полониям, полонии, полониями, полониях",
        ]);
        assert_eq!(decl("магия", "ж 7a"), [
            "магия, магии, магии, магию, магией, магии",
            "магии, магий, магиям, магии, магиями, магиях",
        ]);
        assert_eq!(decl("сложение", "с 7a"), [
            "сложение, сложения, сложению, сложение, сложением, сложении",
            "сложения, сложений, сложениям, сложения, сложениями, сложениях",
        ]);

        // Simple stem type 8
        assert_eq!(decl("путь", "м 8b"), [
            "путь, пути, пути, путь, путём, пути",
            "пути, путей, путям, пути, путями, путях",
        ]);
        assert_eq!(decl("боль", "ж 8a"), [
            "боль, боли, боли, боль, болью, боли",
            "боли, болей, болям, боли, болями, болях",
        ]);
        assert_eq!(decl("брешь", "ж 8a"), [
            "брешь, бреши, бреши, брешь, брешью, бреши",
            "бреши, брешей, брешам, бреши, брешами, брешах",
        ]);
        // All stem type 8 neuters have unique stem alternations

        // assert_eq!(decl("", ""), [
        //     "",
        //     "",
        // ]);
    }

    #[test]
    fn unique_alternation() {
        // -ин
        assert_eq!(decl("крестьянин", "мо 1°a"), [
            "крестьянин, крестьянина, крестьянину, крестьянина, крестьянином, крестьянине",
            "крестьяне, крестьян, крестьянам, крестьян, крестьянами, крестьянах",
        ]);
        assert_eq!(decl("марсианин", "мо 1°a"), [
            "марсианин, марсианина, марсианину, марсианина, марсианином, марсианине",
            "марсиане, марсиан, марсианам, марсиан, марсианами, марсианах",
        ]);
        assert_eq!(decl("боярин", "мо 1°a"), [
            "боярин, боярина, боярину, боярина, боярином, боярине",
            "бояре, бояр, боярам, бояр, боярами, боярах",
        ]);
        assert_eq!(decl("господин", "мо 1°c(1)"), [
            "господин, господина, господину, господина, господином, господине",
            "господа, господ, господам, господ, господами, господах",
        ]);

        // -[оё]нок
        assert_eq!(decl("ребёнок", "мо 3°a"), [
            "ребёнок, ребёнка, ребёнку, ребёнка, ребёнком, ребёнке",
            "ребята, ребят, ребятам, ребят, ребятами, ребятах",
        ]);
        assert_eq!(decl("зайчонок", "мо 3°a"), [
            "зайчонок, зайчонка, зайчонку, зайчонка, зайчонком, зайчонке",
            "зайчата, зайчат, зайчатам, зайчат, зайчатами, зайчатах",
        ]);

        // -ок
        assert_eq!(decl("щенок", "мо 3°d"), [
            "щенок, щенка, щенку, щенка, щенком, щенке",
            "щенята, щенят, щенятам, щенят, щенятами, щенятах",
        ]);
        assert_eq!(decl("внучок", "мо 3°b"), [
            "внучок, внучка, внучку, внучка, внучком, внучке",
            "внучата, внучат, внучатам, внучат, внучатами, внучатах",
        ]);

        // -[оё]ночек
        assert_eq!(decl("поросёночек", "мо 3°a"), [
            "поросёночек, поросёночка, поросёночку, поросёночка, поросёночком, поросёночке",
            "поросятки, поросяток, поросяткам, поросяток, поросятками, поросятках",
        ]);
        assert_eq!(decl("мышоночек", "мо 3°a"), [
            "мышоночек, мышоночка, мышоночку, мышоночка, мышоночком, мышоночке",
            "мышатки, мышаток, мышаткам, мышаток, мышатками, мышатках",
        ]);

        // -очек
        assert_eq!(decl("щеночек", "мо 3°d"), [
            "щеночек, щеночка, щеночку, щеночка, щеночком, щеночке",
            "щенятки, щеняток, щеняткам, щеняток, щенятками, щенятках",
        ]);
        assert_eq!(decl("внучочек", "мо 3°b"), [
            "внучочек, внучочка, внучочку, внучочка, внучочком, внучочке",
            "внучатки, внучаток, внучаткам, внучаток, внучатками, внучатках",
        ]);

        // -мя
        assert_eq!(decl("время", "с 8°c, ё"), [
            "время, времени, времени, время, временем, времени",
            "времена, времён, временам, времена, временами, временах",
        ]);
        assert_eq!(decl("имя", "с 8°c, ё"), [
            "имя, имени, имени, имя, именем, имени",
            "имена, имён, именам, имена, именами, именах",
        ]);

        // assert_eq!(decl("", ""), [
        //     "",
        //     "",
        // ]);
    }

    #[test]
    fn vowel_alternation() {
        // Vowel alternation type A (masc any / fem 8*)
        assert_eq!(decl("сон", "м 1*b"), [
            "сон, сна, сну, сон, сном, сне",
            "сны, снов, снам, сны, снами, снах",
        ]);
        assert_eq!(decl("любовь", "ж 8*b'"), [
            "любовь, любви, любви, любовь, любовью, любви",
            "любви, любвей, любвям, любви, любвями, любвях",
        ]);
        assert_eq!(decl("ветвь", "ж 8e"), [
            "ветвь, ветви, ветви, ветвь, ветвью, ветви",
            "ветви, ветвей, ветвям, ветви, ветвями, ветвях",
        ]);
        assert_eq!(decl("боец", "мо 5*b"), [
            "боец, бойца, бойцу, бойца, бойцом, бойце",
            "бойцы, бойцов, бойцам, бойцов, бойцами, бойцах",
        ]);
        assert_eq!(decl("паёк", "м 3*b"), [
            "паёк, пайка, пайку, паёк, пайком, пайке",
            "пайки, пайков, пайкам, пайки, пайками, пайках",
        ]);
        assert_eq!(decl("улей", "м 6*a"), [
            "улей, улья, улью, улей, ульем, улье",
            "ульи, ульев, ульям, ульи, ульями, ульях",
        ]);
        assert_eq!(decl("зверёк", "мо 3*b"), [
            "зверёк, зверька, зверьку, зверька, зверьком, зверьке",
            "зверьки, зверьков, зверькам, зверьков, зверьками, зверьках",
        ]);
        assert_eq!(decl("лёд", "м 1*b"), [
            "лёд, льда, льду, лёд, льдом, льде",
            "льды, льдов, льдам, льды, льдами, льдах",
        ]);
        assert_eq!(decl("палец", "м 5*a"), [
            "палец, пальца, пальцу, палец, пальцем, пальце",
            "пальцы, пальцев, пальцам, пальцы, пальцами, пальцах",
        ]);
        assert_eq!(decl("орёл", "мо 1*b"), [
            "орёл, орла, орлу, орла, орлом, орле",
            "орлы, орлов, орлам, орлов, орлами, орлах",
        ]);
        assert_eq!(decl("кашель", "м 2*a"), [
            "кашель, кашля, кашлю, кашель, кашлем, кашле",
            "кашли, кашлей, кашлям, кашли, кашлями, кашлях",
        ]);
        assert_eq!(decl("конец", "м 5*b"), [
            "конец, конца, концу, конец, концом, конце",
            "концы, концов, концам, концы, концами, концах",
        ]);

        // Vowel alternation type B (neuter any / fem 1-7*)
        assert_eq!(decl("гостья", "жо 6*a"), [
            "гостья, гостьи, гостье, гостью, гостьей, гостье",
            "гостьи, гостий, гостьям, гостий, гостьями, гостьях",
        ]);
        assert_eq!(decl("ущелье", "с 6*a"), [
            "ущелье, ущелья, ущелью, ущелье, ущельем, ущелье",
            "ущелья, ущелий, ущельям, ущелья, ущельями, ущельях",
        ]);
        assert_eq!(decl("статья", "ж 6*b"), [
            "статья, статьи, статье, статью, статьёй, статье",
            "статьи, статей, статьям, статьи, статьями, статьях",
        ]);
        assert_eq!(decl("питьё", "с 6*b"), [
            "питьё, питья, питью, питьё, питьём, питье",
            "питья, питей, питьям, питья, питьями, питьях",
        ]);
        assert_eq!(decl("шпилька", "ж 3*a"), [
            "шпилька, шпильки, шпильке, шпильку, шпилькой, шпильке",
            "шпильки, шпилек, шпилькам, шпильки, шпильками, шпильках",
        ]);
        assert_eq!(decl("письмо", "с 1*d"), [
            "письмо, письма, письму, письмо, письмом, письме",
            "письма, писем, письмам, письма, письмами, письмах",
        ]);
        assert_eq!(decl("чайка", "жо 3*a"), [
            "чайка, чайки, чайке, чайку, чайкой, чайке",
            "чайки, чаек, чайкам, чаек, чайками, чайках",
        ]);
        assert_eq!(decl("серьга", "ж 3*f"), [
            "серьга, серьги, серьге, серьгу, серьгой, серьге",
            "серьги, серёг, серьгам, серьги, серьгами, серьгах",
        ]);
        assert_eq!(decl("кайма", "ж 1*b"), [
            "кайма, каймы, кайме, кайму, каймой, кайме",
            "каймы, каём, каймам, каймы, каймами, каймах",
        ]);
        assert_eq!(decl("кольцо", "с 5*d"), [
            // Note: Genitive Plural form has anomalous stress
            "кольцо, кольца, кольцу, кольцо, кольцом, кольце",
            "кольца, колец, кольцам, кольца, кольцами, кольцах",
        ]);
        assert_eq!(decl("кукла", "ж 1*a"), [
            "кукла, куклы, кукле, куклу, куклой, кукле",
            "куклы, кукол, куклам, куклы, куклами, куклах",
        ]);
        assert_eq!(decl("окно", "с 1*d"), [
            "окно, окна, окну, окно, окном, окне",
            "окна, окон, окнам, окна, окнами, окнах",
        ]);
        assert_eq!(decl("сказка", "ж 3*a"), [
            "сказка, сказки, сказке, сказку, сказкой, сказке",
            "сказки, сказок, сказкам, сказки, сказками, сказках",
        ]);
        assert_eq!(decl("сосна", "ж 1*d"), [
            "сосна, сосны, сосне, сосну, сосной, сосне",
            "сосны, сосен, соснам, сосны, соснами, соснах",
        ]);
        assert_eq!(decl("число", "с 1*d"), [
            "число, числа, числу, число, числом, числе",
            "числа, чисел, числам, числа, числами, числах",
        ]);
        assert_eq!(decl("ножна", "ж 1*a"), [
            "ножна, ножны, ножне, ножну, ножной, ножне",
            "ножны, ножен, ножнам, ножны, ножнами, ножнах",
        ]);
        assert_eq!(decl("кишка", "ж 3*b"), [
            "кишка, кишки, кишке, кишку, кишкой, кишке",
            "кишки, кишок, кишкам, кишки, кишками, кишках",
        ]);
        assert_eq!(decl("овца", "жо 5*d"), [
            // Note: Genitive/Accusative Plural form has anomalous stress
            "овца, овцы, овце, овцу, овцой, овце",
            "овцы, овец, овцам, овец, овцами, овцах",
        ]);

        assert_eq!(decl("кошка", "жо 3*a"), [
            "кошка, кошки, кошке, кошку, кошкой, кошке",
            "кошки, кошек, кошкам, кошек, кошками, кошках",
        ]);
        assert_eq!(decl("платок", "м 3*b"), [
            "платок, платка, платку, платок, платком, платке",
            "платки, платков, платкам, платки, платками, платках",
        ]);
        assert_eq!(decl("сердце", "с 5*c"), [
            "сердце, сердца, сердцу, сердце, сердцем, сердце",
            "сердца, сердец, сердцам, сердца, сердцами, сердцах",
        ]);

        // Special case for feminine 2*a, ending with 'ня'
        assert_eq!(decl("башня", "ж 2*a"), [
            "башня, башни, башне, башню, башней, башне",
            "башни, башен, башням, башни, башнями, башнях",
        ]);
        assert_eq!(decl("пекарня", "ж 2*a"), [
            "пекарня, пекарни, пекарне, пекарню, пекарней, пекарне",
            "пекарни, пекарен, пекарням, пекарни, пекарнями, пекарнях",
        ]);

        // Exceptions 2*b, 2*f, and (2)
        assert_eq!(decl("лыжня", "ж 2*b"), [
            "лыжня, лыжни, лыжне, лыжню, лыжнёй, лыжне",
            "лыжни, лыжней, лыжням, лыжни, лыжнями, лыжнях",
        ]);
        assert_eq!(decl("сходня", "ж 2*a(2)"), [
            "сходня, сходни, сходне, сходню, сходней, сходне",
            "сходни, сходней, сходням, сходни, сходнями, сходнях",
        ]);
        assert_eq!(decl("платье", "с 6*a(2)"), [
            "платье, платья, платью, платье, платьем, платье",
            "платья, платьев, платьям, платья, платьями, платьях",
        ]);
        assert_eq!(decl("облачко", "с 3*c(2)"), [
            "облачко, облачка, облачку, облачко, облачком, облачке",
            "облачка, облачков, облачкам, облачка, облачками, облачках",
        ]);
        assert_eq!(decl("жальце", "с 5*a"), [
            "жальце, жальца, жальцу, жальце, жальцем, жальце",
            "жальца, жалец, жальцам, жальца, жальцами, жальцах",
        ]);
        assert_eq!(decl("жальце", "с 5*a(2)"), [
            "жальце, жальца, жальцу, жальце, жальцем, жальце",
            "жальца, жальцев, жальцам, жальца, жальцами, жальцах",
        ]);

        // assert_eq!(decl("", ""), [
        //     "",
        //     "",
        // ]);
    }

    #[test]
    fn ye_yo_alternation() {
        // Alternation with ё in the stem
        assert_eq!(decl("ёж", "мо 4b, ё"), [
            "ёж, ежа, ежу, ежа, ежом, еже",
            "ежи, ежей, ежам, ежей, ежами, ежах",
        ]);
        assert_eq!(decl("делёж", "мо 4b, ё"), [
            "делёж, дележа, дележу, дележа, дележом, дележе",
            "дележи, дележей, дележам, дележей, дележами, дележах",
        ]);
        assert_eq!(decl("черёд", "м 1b, ё"), [
            "черёд, череда, череду, черёд, чередом, череде",
            "череды, чередов, чередам, череды, чередами, чередах",
        ]);
        assert_eq!(decl("шёлк", "м 3c(1), ё"), [
            "шёлк, шёлка, шёлку, шёлк, шёлком, шёлке",
            "шелка, шелков, шелкам, шелка, шелками, шелках",
        ]);
        assert_eq!(decl("жёлудь", "м 2e, ё"), [
            "жёлудь, жёлудя, жёлудю, жёлудь, жёлудем, жёлуде",
            "жёлуди, желудей, желудям, жёлуди, желудями, желудях",
        ]);
        assert_eq!(decl("щёлочь", "ж 8e, ё"), [
            "щёлочь, щёлочи, щёлочи, щёлочь, щёлочью, щёлочи",
            "щёлочи, щелочей, щелочам, щёлочи, щелочами, щелочах",
        ]);

        // Alternation without ё in the stem
        assert_eq!(decl("стега", "ж 3b, ё"), [
            "стега, стеги, стеге, стегу, стегой, стеге",
            "стеги, стёг, стегам, стеги, стегами, стегах",
        ]);
        assert_eq!(decl("середа", "ж 1f', ё"), [
            "середа, середы, середе, середу, середой, середе",
            "середы, серёд, середам, середы, середами, середах",
        ]);
        assert_eq!(decl("череда", "ж 1b, ё"), [
            "череда, череды, череде, череду, чередой, череде",
            "череды, черёд, чередам, череды, чередами, чередах",
        ]);
        assert_eq!(decl("село", "с 1d, ё"), [
            "село, села, селу, село, селом, селе",
            "сёла, сёл, сёлам, сёла, сёлами, сёлах",
        ]);
        assert_eq!(decl("веретено", "с 1d, ё"), [
            "веретено, веретена, веретену, веретено, веретеном, веретене",
            "веретёна, веретён, веретёнам, веретёна, веретёнами, веретёнах",
        ]);
        assert_eq!(decl("жена", "жо 1d, ё"), [
            "жена, жены, жене, жену, женой, жене",
            "жёны, жён, жёнам, жён, жёнами, жёнах",
        ]);
        assert_eq!(decl("слеза", "ж 1f, ё"), [
            "слеза, слезы, слезе, слезу, слезой, слезе",
            "слёзы, слёз, слезам, слёзы, слезами, слезах",
        ]);
        assert_eq!(decl("железа", "ж 1f, ё"), [
            "железа, железы, железе, железу, железой, железе",
            "железы, желёз, железам, железы, железами, железах",
        ]);

        // Alternation without ё in the stem, and also with alternating vowels
        assert_eq!(decl("стекло", "с 1*d, ё"), [
            "стекло, стекла, стеклу, стекло, стеклом, стекле",
            "стёкла, стёкол, стёклам, стёкла, стёклами, стёклах",
        ]);
        assert_eq!(decl("бронестекло", "с 1*d, ё"), [
            "бронестекло, бронестекла, бронестеклу, бронестекло, бронестеклом, бронестекле",
            "бронестёкла, бронестёкол, бронестёклам, бронестёкла, бронестёклами, бронестёклах",
        ]);
        assert_eq!(decl("метла", "ж 1*d, ё"), [
            "метла, метлы, метле, метлу, метлой, метле",
            "мётлы, мётел, мётлам, мётлы, мётлами, мётлах",
        ]);
        assert_eq!(decl("сестра", "жо 1*d, ё"), [
            // Note: Genitive Plural form is anomalous (сестёр), but we'll ignore that here
            "сестра, сестры, сестре, сестру, сестрой, сестре",
            "сёстры, сёстер, сёстрам, сёстер, сёстрами, сёстрах",
        ]);

        // assert_eq!(decl("", ""), [
        //     "",
        //     "",
        // ]);
    }

    #[test]
    fn circled_digits() {
        // Circled one, ①
        assert_eq!(decl("жвало", "с 1a(1)"), [
            "жвало, жвала, жвалу, жвало, жвалом, жвале",
            "жвалы, жвал, жвалам, жвалы, жвалами, жвалах",
        ]);
        assert_eq!(decl("горлышко", "с 3*a(1)"), [
            "горлышко, горлышка, горлышку, горлышко, горлышком, горлышке",
            "горлышки, горлышек, горлышкам, горлышки, горлышками, горлышках",
        ]);
        assert_eq!(decl("ветер", "м 1*c(1)"), [
            "ветер, ветра, ветру, ветер, ветром, ветре",
            "ветра, ветров, ветрам, ветра, ветрами, ветрах",
        ]);
        assert_eq!(decl("край", "м 6c(1)"), [
            "край, края, краю, край, краем, крае",
            "края, краёв, краям, края, краями, краях",
        ]);

        // Circled two, ②
        assert_eq!(decl("облако", "с 3c(2)"), [
            "облако, облака, облаку, облако, облаком, облаке",
            "облака, облаков, облакам, облака, облаками, облаках",
        ]);
        assert_eq!(decl("оконце", "с 5*a(2)"), [
            "оконце, оконца, оконцу, оконце, оконцем, оконце",
            "оконца, оконцев, оконцам, оконца, оконцами, оконцах",
        ]);
        assert_eq!(decl("мясцо", "с 5*b(2)"), [
            "мясцо, мясца, мясцу, мясцо, мясцом, мясце",
            "мясца, мясцов, мясцам, мясца, мясцами, мясцах",
        ]);
        assert_eq!(decl("подполье", "с 6*a(2)"), [
            "подполье, подполья, подполью, подполье, подпольем, подполье",
            "подполья, подпольев, подпольям, подполья, подпольями, подпольях",
        ]);
        assert_eq!(decl("жнивьё", "с 6*b(2)"), [
            "жнивьё, жнивья, жнивью, жнивьё, жнивьём, жнивье",
            "жнивья, жнивьёв, жнивьям, жнивья, жнивьями, жнивьях",
        ]);
        assert_eq!(decl("байт", "м 1a(2)"), [
            "байт, байта, байту, байт, байтом, байте",
            "байты, байт, байтам, байты, байтами, байтах",
        ]);
        // Note: There are no masculine nouns with ②, of type 2/6/7/8?
        assert_eq!(decl("ноздря", "ж 2f(2)"), [
            "ноздря, ноздри, ноздре, ноздрю, ноздрёй, ноздре",
            "ноздри, ноздрей, ноздрям, ноздри, ноздрями, ноздрях",
        ]);

        // Circled one and two, ①②
        assert_eq!(decl("глаз", "м 1c(1)(2)"), [
            "глаз, глаза, глазу, глаз, глазом, глазе",
            "глаза, глаз, глазам, глаза, глазами, глазах",
        ]);
        assert_eq!(decl("очко", "с 3*b(1)(2)"), [
            "очко, очка, очку, очко, очком, очке",
            "очки, очков, очкам, очки, очками, очках",
        ]);

        // Circled three, ③
        assert_eq!(decl("чий", "м 7a(3)"), [
            "чий, чия, чию, чий, чием, чие",
            "чии, чиев, чиям, чии, чиями, чиях",
        ]);
        assert_eq!(decl("лия", "жо 7a(3)"), [
            "лия, лии, лие, лию, лией, лие",
            "лии, лий, лиям, лий, лиями, лиях",
        ]);

        // assert_eq!(decl("", ""), [
        //     "",
        //     "",
        // ]);
    }
}
