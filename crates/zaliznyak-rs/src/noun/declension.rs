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
}

impl NounInfo {
    pub fn inflect(&self, stem: Word, case: CaseEx, number: Number) -> WordBuf {
        let mut word = WordBuf::with_stem(stem, 5);
        let mut buf = InflectionBuf::new(&mut word);

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

        buf.finish(&mut word);
        word
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

        // TODO: Move the stress to the ending, if needed
        if self.stress.is_ending_stressed(info) {
            if let Some(ending_pos) = buf.ending().iter().position(|x| x.is_vowel()) {
                buf.stress_at = buf.stem_len + ending_pos + 1;
            } else {
                buf.stress_at = buf.stem().iter().rposition(|x| x.is_vowel()).unwrap() + 1;
            }
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
                buf.set_stress_at(ye);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::word::Accent;

    fn decl(word: &str, info: &str) -> [String; 2] {
        let mut stem: WordBuf = word.parse().unwrap();
        let _ty = NounStemType::identify_trim(&mut stem);

        let noun = Noun { stem, info: info.parse().unwrap() };

        Number::VALUES.map(|number| {
            Case::VALUES
                .map(|case| {
                    let word = noun.inflect(case.into(), number);
                    word.display().accent(Accent::explicit(Accent::ACUTE)).to_string()
                })
                .join(", ")
        })
    }

    #[test]
    fn simple_stem_types() {
        // Simple stem type 1
        assert_eq!(decl("топо́р", "м 1b"), [
            "топо́р, топора́, топору́, топо́р, топоро́м, топоре́",
            "топоры́, топоро́в, топора́м, топоры́, топора́ми, топора́х",
        ]);
        assert_eq!(decl("ко́бра", "жо 1a"), [
            "ко́бра, ко́бры, ко́бре, ко́бру, ко́брой, ко́бре",
            "ко́бры, ко́бр, ко́брам, ко́бр, ко́брами, ко́брах",
        ]);
        assert_eq!(decl("о́лово", "с 1a"), [
            "о́лово, о́лова, о́лову, о́лово, о́ловом, о́лове",
            "о́лова, о́лов, о́ловам, о́лова, о́ловами, о́ловах",
        ]);

        // Simple stem type 2
        assert_eq!(decl("иска́тель", "мо 2a"), [
            "иска́тель, иска́теля, иска́телю, иска́теля, иска́телем, иска́теле",
            "иска́тели, иска́телей, иска́телям, иска́телей, иска́телями, иска́телях",
        ]);
        assert_eq!(decl("ба́ня", "ж 2a"), [
            "ба́ня, ба́ни, ба́не, ба́ню, ба́ней, ба́не",
            "ба́ни, ба́нь, ба́ням, ба́ни, ба́нями, ба́нях",
        ]);
        assert_eq!(decl("по́ле", "с 2c"), [
            "по́ле, по́ля, по́лю, по́ле, по́лем, по́ле",
            "поля́, поле́й, поля́м, поля́, поля́ми, поля́х",
        ]);

        // Simple stem type 3
        assert_eq!(decl("бли́нчик", "мо 3a"), [
            "бли́нчик, бли́нчика, бли́нчику, бли́нчика, бли́нчиком, бли́нчике",
            "бли́нчики, бли́нчиков, бли́нчикам, бли́нчиков, бли́нчиками, бли́нчиках",
        ]);
        assert_eq!(decl("коря́га", "ж 3a"), [
            "коря́га, коря́ги, коря́ге, коря́гу, коря́гой, коря́ге",
            "коря́ги, коря́г, коря́гам, коря́ги, коря́гами, коря́гах",
        ]);
        assert_eq!(decl("во́йско", "с 3c"), [
            "во́йско, во́йска, во́йску, во́йско, во́йском, во́йске",
            "войска́, во́йск, войска́м, войска́, войска́ми, войска́х",
        ]);

        // Simple stem type 4
        assert_eq!(decl("кала́ч", "м 4b"), [
            "кала́ч, калача́, калачу́, кала́ч, калачо́м, калаче́",
            "калачи́, калаче́й, калача́м, калачи́, калача́ми, калача́х",
        ]);
        assert_eq!(decl("гало́ша", "ж 4a"), [
            "гало́ша, гало́ши, гало́ше, гало́шу, гало́шей, гало́ше",
            "гало́ши, гало́ш, гало́шам, гало́ши, гало́шами, гало́шах",
        ]);
        assert_eq!(decl("жили́ще", "с 4a"), [
            "жили́ще, жили́ща, жили́щу, жили́ще, жили́щем, жили́ще",
            "жили́ща, жили́щ, жили́щам, жили́ща, жили́щами, жили́щах",
        ]);

        // Simple stem type 5
        assert_eq!(decl("кузне́ц", "мо 5b"), [
            "кузне́ц, кузнеца́, кузнецу́, кузнеца́, кузнецо́м, кузнеце́",
            "кузнецы́, кузнецо́в, кузнеца́м, кузнецо́в, кузнеца́ми, кузнеца́х",
        ]);
        assert_eq!(decl("деви́ца", "жо 5a"), [
            "деви́ца, деви́цы, деви́це, деви́цу, деви́цей, деви́це",
            "деви́цы, деви́ц, деви́цам, деви́ц, деви́цами, деви́цах",
        ]);
        assert_eq!(decl("лицо́", "с 5d"), [
            "лицо́, лица́, лицу́, лицо́, лицо́м, лице́",
            "ли́ца, ли́ц, ли́цам, ли́ца, ли́цами, ли́цах",
        ]);

        // Simple stem type 6
        assert_eq!(decl("бо́й", "м 6c"), [
            "бо́й, бо́я, бо́ю, бо́й, бо́ем, бо́е",
            "бои́, боё́в, боя́м, бои́, боя́ми, боя́х",
        ]);
        assert_eq!(decl("ше́я", "ж 6a"), [
            "ше́я, ше́и, ше́е, ше́ю, ше́ей, ше́е",
            "ше́и, ше́й, ше́ям, ше́и, ше́ями, ше́ях",
        ]);
        // All stem type 6 neuters have vowel alternations

        // Simple stem type 7
        assert_eq!(decl("поло́ний", "м 7a"), [
            "поло́ний, поло́ния, поло́нию, поло́ний, поло́нием, поло́нии",
            "поло́нии, поло́ниев, поло́ниям, поло́нии, поло́ниями, поло́ниях",
        ]);
        assert_eq!(decl("ма́гия", "ж 7a"), [
            "ма́гия, ма́гии, ма́гии, ма́гию, ма́гией, ма́гии",
            "ма́гии, ма́гий, ма́гиям, ма́гии, ма́гиями, ма́гиях",
        ]);
        assert_eq!(decl("сложе́ние", "с 7a"), [
            "сложе́ние, сложе́ния, сложе́нию, сложе́ние, сложе́нием, сложе́нии",
            "сложе́ния, сложе́ний, сложе́ниям, сложе́ния, сложе́ниями, сложе́ниях",
        ]);

        // Simple stem type 8
        assert_eq!(decl("пу́ть", "м 8b"), [
            "пу́ть, пути́, пути́, пу́ть, путё́м, пути́",
            "пути́, путе́й, путя́м, пути́, путя́ми, путя́х",
        ]);
        assert_eq!(decl("бо́ль", "ж 8a"), [
            "бо́ль, бо́ли, бо́ли, бо́ль, бо́лью, бо́ли",
            "бо́ли, бо́лей, бо́лям, бо́ли, бо́лями, бо́лях",
        ]);
        assert_eq!(decl("бре́шь", "ж 8a"), [
            "бре́шь, бре́ши, бре́ши, бре́шь, бре́шью, бре́ши",
            "бре́ши, бре́шей, бре́шам, бре́ши, бре́шами, бре́шах",
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
        assert_eq!(decl("крестья́нин", "мо 1°a"), [
            "крестья́нин, крестья́нина, крестья́нину, крестья́нина, крестья́нином, крестья́нине",
            "крестья́не, крестья́н, крестья́нам, крестья́н, крестья́нами, крестья́нах",
        ]);
        assert_eq!(decl("марсиа́нин", "мо 1°a"), [
            "марсиа́нин, марсиа́нина, марсиа́нину, марсиа́нина, марсиа́нином, марсиа́нине",
            "марсиа́не, марсиа́н, марсиа́нам, марсиа́н, марсиа́нами, марсиа́нах",
        ]);
        assert_eq!(decl("боя́рин", "мо 1°a"), [
            "боя́рин, боя́рина, боя́рину, боя́рина, боя́рином, боя́рине",
            "боя́ре, боя́р, боя́рам, боя́р, боя́рами, боя́рах",
        ]);
        assert_eq!(decl("господи́н", "мо 1°c(1)"), [
            "господи́н, господи́на, господи́ну, господи́на, господи́ном, господи́не",
            "господа́, госпо́д, господа́м, госпо́д, господа́ми, господа́х",
        ]);

        // -[оё]нок
        assert_eq!(decl("ребё́нок", "мо 3°a"), [
            "ребё́нок, ребё́нка, ребё́нку, ребё́нка, ребё́нком, ребё́нке",
            "ребя́та, ребя́т, ребя́там, ребя́т, ребя́тами, ребя́тах",
        ]);
        assert_eq!(decl("зайчо́нок", "мо 3°a"), [
            "зайчо́нок, зайчо́нка, зайчо́нку, зайчо́нка, зайчо́нком, зайчо́нке",
            "зайча́та, зайча́т, зайча́там, зайча́т, зайча́тами, зайча́тах",
        ]);

        // -ок
        assert_eq!(decl("щено́к", "мо 3°d"), [
            "щено́к, щенка́, щенку́, щенка́, щенко́м, щенке́",
            "щеня́та, щеня́т, щеня́там, щеня́т, щеня́тами, щеня́тах",
        ]);
        assert_eq!(decl("внучо́к", "мо 3°b"), [
            "внучо́к, внучка́, внучку́, внучка́, внучко́м, внучке́",
            "внуча́та, внуча́т, внуча́там, внуча́т, внуча́тами, внуча́тах",
        ]);

        // -[оё]ночек
        assert_eq!(decl("поросё́ночек", "мо 3°a"), [
            "поросё́ночек, поросё́ночка, поросё́ночку, поросё́ночка, поросё́ночком, поросё́ночке",
            "порося́тки, порося́ток, порося́ткам, порося́ток, порося́тками, порося́тках",
        ]);
        assert_eq!(decl("мышо́ночек", "мо 3°a"), [
            "мышо́ночек, мышо́ночка, мышо́ночку, мышо́ночка, мышо́ночком, мышо́ночке",
            "мыша́тки, мыша́ток, мыша́ткам, мыша́ток, мыша́тками, мыша́тках",
        ]);

        // -очек
        assert_eq!(decl("щено́чек", "мо 3°d"), [
            "щено́чек, щено́чка, щено́чку, щено́чка, щено́чком, щено́чке",
            "щеня́тки, щеня́ток, щеня́ткам, щеня́ток, щеня́тками, щеня́тках",
        ]);
        assert_eq!(decl("внучо́чек", "мо 3°b"), [
            "внучо́чек, внучо́чка, внучо́чку, внучо́чка, внучо́чком, внучо́чке",
            "внуча́тки, внуча́ток, внуча́ткам, внуча́ток, внуча́тками, внуча́тках",
        ]);

        // -мя
        assert_eq!(decl("вре́мя", "с 8°c, ё"), [
            "вре́мя, вре́мени, вре́мени, вре́мя, вре́менем, вре́мени",
            "времена́, времё́н, времена́м, времена́, времена́ми, времена́х",
        ]);
        assert_eq!(decl("и́мя", "с 8°c, ё"), [
            "и́мя, и́мени, и́мени, и́мя, и́менем, и́мени",
            "имена́, имё́н, имена́м, имена́, имена́ми, имена́х",
        ]);

        // assert_eq!(decl("", ""), [
        //     "",
        //     "",
        // ]);
    }

    #[test]
    fn vowel_alternation() {
        // Vowel alternation type A (masc any / fem 8*)
        assert_eq!(decl("со́н", "м 1*b"), [
            "со́н, сна́, сну́, со́н, сно́м, сне́",
            "сны́, сно́в, сна́м, сны́, сна́ми, сна́х",
        ]);
        assert_eq!(decl("любо́вь", "ж 8*b'"), [
            "любо́вь, любви́, любви́, любо́вь, любо́вью, любви́",
            "любви́, любве́й, любвя́м, любви́, любвя́ми, любвя́х",
        ]);
        assert_eq!(decl("ве́твь", "ж 8e"), [
            "ве́твь, ве́тви, ве́тви, ве́твь, ве́твью, ве́тви",
            "ве́тви, ветве́й, ветвя́м, ве́тви, ветвя́ми, ветвя́х",
        ]);
        assert_eq!(decl("бое́ц", "мо 5*b"), [
            "бое́ц, бойца́, бойцу́, бойца́, бойцо́м, бойце́",
            "бойцы́, бойцо́в, бойца́м, бойцо́в, бойца́ми, бойца́х",
        ]);
        assert_eq!(decl("паё́к", "м 3*b"), [
            "паё́к, пайка́, пайку́, паё́к, пайко́м, пайке́",
            "пайки́, пайко́в, пайка́м, пайки́, пайка́ми, пайка́х",
        ]);
        assert_eq!(decl("у́лей", "м 6*a"), [
            "у́лей, у́лья, у́лью, у́лей, у́льем, у́лье",
            "у́льи, у́льев, у́льям, у́льи, у́льями, у́льях",
        ]);
        assert_eq!(decl("зверё́к", "мо 3*b"), [
            "зверё́к, зверька́, зверьку́, зверька́, зверько́м, зверьке́",
            "зверьки́, зверько́в, зверька́м, зверько́в, зверька́ми, зверька́х",
        ]);
        assert_eq!(decl("лё́д", "м 1*b"), [
            "лё́д, льда́, льду́, лё́д, льдо́м, льде́",
            "льды́, льдо́в, льда́м, льды́, льда́ми, льда́х",
        ]);
        assert_eq!(decl("па́лец", "м 5*a"), [
            "па́лец, па́льца, па́льцу, па́лец, па́льцем, па́льце",
            "па́льцы, па́льцев, па́льцам, па́льцы, па́льцами, па́льцах",
        ]);
        assert_eq!(decl("орё́л", "мо 1*b"), [
            "орё́л, орла́, орлу́, орла́, орло́м, орле́",
            "орлы́, орло́в, орла́м, орло́в, орла́ми, орла́х",
        ]);
        assert_eq!(decl("ка́шель", "м 2*a"), [
            "ка́шель, ка́шля, ка́шлю, ка́шель, ка́шлем, ка́шле",
            "ка́шли, ка́шлей, ка́шлям, ка́шли, ка́шлями, ка́шлях",
        ]);
        assert_eq!(decl("коне́ц", "м 5*b"), [
            "коне́ц, конца́, концу́, коне́ц, концо́м, конце́",
            "концы́, концо́в, конца́м, концы́, конца́ми, конца́х",
        ]);

        // Vowel alternation type B (neuter any / fem 1-7*)
        assert_eq!(decl("го́стья", "жо 6*a"), [
            "го́стья, го́стьи, го́стье, го́стью, го́стьей, го́стье",
            "го́стьи, го́стий, го́стьям, го́стий, го́стьями, го́стьях",
        ]);
        assert_eq!(decl("уще́лье", "с 6*a"), [
            "уще́лье, уще́лья, уще́лью, уще́лье, уще́льем, уще́лье",
            "уще́лья, уще́лий, уще́льям, уще́лья, уще́льями, уще́льях",
        ]);
        assert_eq!(decl("статья́", "ж 6*b"), [
            "статья́, статьи́, статье́, статью́, статьё́й, статье́",
            "статьи́, стате́й, статья́м, статьи́, статья́ми, статья́х",
        ]);
        assert_eq!(decl("питьё́", "с 6*b"), [
            "питьё́, питья́, питью́, питьё́, питьё́м, питье́",
            "питья́, пите́й, питья́м, питья́, питья́ми, питья́х",
        ]);
        assert_eq!(decl("шпи́лька", "ж 3*a"), [
            "шпи́лька, шпи́льки, шпи́льке, шпи́льку, шпи́лькой, шпи́льке",
            "шпи́льки, шпи́лек, шпи́лькам, шпи́льки, шпи́льками, шпи́льках",
        ]);
        assert_eq!(decl("письмо́", "с 1*d"), [
            "письмо́, письма́, письму́, письмо́, письмо́м, письме́",
            "пи́сьма, пи́сем, пи́сьмам, пи́сьма, пи́сьмами, пи́сьмах",
        ]);
        assert_eq!(decl("ча́йка", "жо 3*a"), [
            "ча́йка, ча́йки, ча́йке, ча́йку, ча́йкой, ча́йке",
            "ча́йки, ча́ек, ча́йкам, ча́ек, ча́йками, ча́йках",
        ]);
        assert_eq!(decl("серьга́", "ж 3*f"), [
            "серьга́, серьги́, серьге́, серьгу́, серьго́й, серьге́",
            "се́рьги, серё́г, серьга́м, се́рьги, серьга́ми, серьга́х",
        ]);
        assert_eq!(decl("кайма́", "ж 1*b"), [
            "кайма́, каймы́, кайме́, кайму́, каймо́й, кайме́",
            "каймы́, каё́м, кайма́м, каймы́, кайма́ми, кайма́х",
        ]);
        assert_eq!(decl("кольцо́", "с 5*d"), [
            // Note: Genitive Plural form has anomalous stress (коле́ц), but we'll ignore that here
            "кольцо́, кольца́, кольцу́, кольцо́, кольцо́м, кольце́",
            "ко́льца, ко́лец, ко́льцам, ко́льца, ко́льцами, ко́льцах",
        ]);
        assert_eq!(decl("ку́кла", "ж 1*a"), [
            "ку́кла, ку́клы, ку́кле, ку́клу, ку́клой, ку́кле",
            "ку́клы, ку́кол, ку́клам, ку́клы, ку́клами, ку́клах",
        ]);
        assert_eq!(decl("окно́", "с 1*d"), [
            "окно́, окна́, окну́, окно́, окно́м, окне́",
            "о́кна, о́кон, о́кнам, о́кна, о́кнами, о́кнах",
        ]);
        assert_eq!(decl("ска́зка", "ж 3*a"), [
            "ска́зка, ска́зки, ска́зке, ска́зку, ска́зкой, ска́зке",
            "ска́зки, ска́зок, ска́зкам, ска́зки, ска́зками, ска́зках",
        ]);
        assert_eq!(decl("сосна́", "ж 1*d"), [
            "сосна́, сосны́, сосне́, сосну́, сосно́й, сосне́",
            "со́сны, со́сен, со́снам, со́сны, со́снами, со́снах",
        ]);
        assert_eq!(decl("число́", "с 1*d"), [
            "число́, числа́, числу́, число́, число́м, числе́",
            "чи́сла, чи́сел, чи́слам, чи́сла, чи́слами, чи́слах",
        ]);
        assert_eq!(decl("но́жна", "ж 1*a"), [
            "но́жна, но́жны, но́жне, но́жну, но́жной, но́жне",
            "но́жны, но́жен, но́жнам, но́жны, но́жнами, но́жнах",
        ]);
        assert_eq!(decl("кишка́", "ж 3*b"), [
            "кишка́, кишки́, кишке́, кишку́, кишко́й, кишке́",
            "кишки́, кишо́к, кишка́м, кишки́, кишка́ми, кишка́х",
        ]);
        assert_eq!(decl("овца́", "жо 5*d"), [
            // Note: Genitive/Accusative Plural form has anomalous stress (ове́ц), but we'll ignore that here
            "овца́, овцы́, овце́, овцу́, овцо́й, овце́",
            "о́вцы, о́вец, о́вцам, о́вец, о́вцами, о́вцах",
        ]);

        assert_eq!(decl("ко́шка", "жо 3*a"), [
            "ко́шка, ко́шки, ко́шке, ко́шку, ко́шкой, ко́шке",
            "ко́шки, ко́шек, ко́шкам, ко́шек, ко́шками, ко́шках",
        ]);
        assert_eq!(decl("плато́к", "м 3*b"), [
            "плато́к, платка́, платку́, плато́к, платко́м, платке́",
            "платки́, платко́в, платка́м, платки́, платка́ми, платка́х",
        ]);
        assert_eq!(decl("се́рдце", "с 5*c"), [
            "се́рдце, се́рдца, се́рдцу, се́рдце, се́рдцем, се́рдце",
            "сердца́, серде́ц, сердца́м, сердца́, сердца́ми, сердца́х",
        ]);

        // Special case for feminine 2*a, ending with 'ня'
        assert_eq!(decl("ба́шня", "ж 2*a"), [
            "ба́шня, ба́шни, ба́шне, ба́шню, ба́шней, ба́шне",
            "ба́шни, ба́шен, ба́шням, ба́шни, ба́шнями, ба́шнях",
        ]);
        assert_eq!(decl("пека́рня", "ж 2*a"), [
            "пека́рня, пека́рни, пека́рне, пека́рню, пека́рней, пека́рне",
            "пека́рни, пека́рен, пека́рням, пека́рни, пека́рнями, пека́рнях",
        ]);

        // Exceptions 2*b, 2*f, and (2)
        assert_eq!(decl("лыжня́", "ж 2*b"), [
            "лыжня́, лыжни́, лыжне́, лыжню́, лыжнё́й, лыжне́",
            "лыжни́, лыжне́й, лыжня́м, лыжни́, лыжня́ми, лыжня́х",
        ]);
        assert_eq!(decl("схо́дня", "ж 2*a(2)"), [
            "схо́дня, схо́дни, схо́дне, схо́дню, схо́дней, схо́дне",
            "схо́дни, схо́дней, схо́дням, схо́дни, схо́днями, схо́днях",
        ]);
        assert_eq!(decl("пла́тье", "с 6*a(2)"), [
            "пла́тье, пла́тья, пла́тью, пла́тье, пла́тьем, пла́тье",
            "пла́тья, пла́тьев, пла́тьям, пла́тья, пла́тьями, пла́тьях",
        ]);
        assert_eq!(decl("о́блачко", "с 3*c(2)"), [
            "о́блачко, о́блачка, о́блачку, о́блачко, о́блачком, о́блачке",
            "облачка́, облачко́в, облачка́м, облачка́, облачка́ми, облачка́х",
        ]);
        assert_eq!(decl("жа́льце", "с 5*a"), [
            "жа́льце, жа́льца, жа́льцу, жа́льце, жа́льцем, жа́льце",
            "жа́льца, жа́лец, жа́льцам, жа́льца, жа́льцами, жа́льцах",
        ]);
        assert_eq!(decl("жа́льце", "с 5*a(2)"), [
            "жа́льце, жа́льца, жа́льцу, жа́льце, жа́льцем, жа́льце",
            "жа́льца, жа́льцев, жа́льцам, жа́льца, жа́льцами, жа́льцах",
        ]);

        // assert_eq!(decl("", ""), [
        //     "",
        //     "",
        // ]);
    }

    #[test]
    fn ye_yo_alternation() {
        // Alternation with ё in the stem
        assert_eq!(decl("ё́ж", "мо 4b, ё"), [
            "ё́ж, ежа́, ежу́, ежа́, ежо́м, еже́",
            "ежи́, еже́й, ежа́м, еже́й, ежа́ми, ежа́х",
        ]);
        assert_eq!(decl("делё́ж", "мо 4b, ё"), [
            "делё́ж, дележа́, дележу́, дележа́, дележо́м, дележе́",
            "дележи́, дележе́й, дележа́м, дележе́й, дележа́ми, дележа́х",
        ]);
        assert_eq!(decl("черё́д", "м 1b, ё"), [
            "черё́д, череда́, череду́, черё́д, чередо́м, череде́",
            "череды́, чередо́в, череда́м, череды́, череда́ми, череда́х",
        ]);
        assert_eq!(decl("шё́лк", "м 3c(1), ё"), [
            "шё́лк, шё́лка, шё́лку, шё́лк, шё́лком, шё́лке",
            "шелка́, шелко́в, шелка́м, шелка́, шелка́ми, шелка́х",
        ]);
        assert_eq!(decl("жё́лудь", "м 2e, ё"), [
            "жё́лудь, жё́лудя, жё́лудю, жё́лудь, жё́лудем, жё́луде",
            "жё́луди, желуде́й, желудя́м, жё́луди, желудя́ми, желудя́х",
        ]);
        assert_eq!(decl("щё́лочь", "ж 8e, ё"), [
            "щё́лочь, щё́лочи, щё́лочи, щё́лочь, щё́лочью, щё́лочи",
            "щё́лочи, щелоче́й, щелоча́м, щё́лочи, щелоча́ми, щелоча́х",
        ]);

        // Alternation without ё in the stem
        assert_eq!(decl("стега́", "ж 3b, ё"), [
            "стега́, стеги́, стеге́, стегу́, стего́й, стеге́",
            "стеги́, стё́г, стега́м, стеги́, стега́ми, стега́х",
        ]);
        assert_eq!(decl("середа́", "ж 1f', ё"), [
            "середа́, середы́, середе́, се́реду, середо́й, середе́",
            "се́реды, серё́д, середа́м, се́реды, середа́ми, середа́х",
        ]);
        assert_eq!(decl("череда́", "ж 1b, ё"), [
            "череда́, череды́, череде́, череду́, чередо́й, череде́",
            "череды́, черё́д, череда́м, череды́, череда́ми, череда́х",
        ]);
        assert_eq!(decl("село́", "с 1d, ё"), [
            "село́, села́, селу́, село́, село́м, селе́",
            "сё́ла, сё́л, сё́лам, сё́ла, сё́лами, сё́лах",
        ]);
        assert_eq!(decl("веретено́", "с 1d, ё"), [
            "веретено́, веретена́, веретену́, веретено́, веретено́м, веретене́",
            "веретё́на, веретё́н, веретё́нам, веретё́на, веретё́нами, веретё́нах",
        ]);
        assert_eq!(decl("жена́", "жо 1d, ё"), [
            "жена́, жены́, жене́, жену́, жено́й, жене́",
            "жё́ны, жё́н, жё́нам, жё́н, жё́нами, жё́нах",
        ]);
        assert_eq!(decl("слеза́", "ж 1f, ё"), [
            "слеза́, слезы́, слезе́, слезу́, слезо́й, слезе́",
            "слё́зы, слё́з, слеза́м, слё́зы, слеза́ми, слеза́х",
        ]);
        assert_eq!(decl("железа́", "ж 1f, ё"), [
            "железа́, железы́, железе́, железу́, железо́й, железе́",
            "же́лезы, желё́з, железа́м, же́лезы, железа́ми, железа́х",
        ]);

        // Alternation without ё in the stem, and also with alternating vowels
        assert_eq!(decl("стекло́", "с 1*d, ё"), [
            "стекло́, стекла́, стеклу́, стекло́, стекло́м, стекле́",
            "стё́кла, стё́кол, стё́клам, стё́кла, стё́клами, стё́клах",
        ]);
        assert_eq!(decl("бронестекло́", "с 1*d, ё"), [
            "бронестекло́, бронестекла́, бронестеклу́, бронестекло́, бронестекло́м, бронестекле́",
            "бронестё́кла, бронестё́кол, бронестё́клам, бронестё́кла, бронестё́клами, бронестё́клах",
        ]);
        assert_eq!(decl("метла́", "ж 1*d, ё"), [
            "метла́, метлы́, метле́, метлу́, метло́й, метле́",
            "мё́тлы, мё́тел, мё́тлам, мё́тлы, мё́тлами, мё́тлах",
        ]);
        assert_eq!(decl("сестра́", "жо 1*d, ё"), [
            // Note: Genitive Plural form is anomalous (сестёр), but we'll ignore that here
            "сестра́, сестры́, сестре́, сестру́, сестро́й, сестре́",
            "сё́стры, сё́стер, сё́страм, сё́стер, сё́страми, сё́страх",
        ]);

        // assert_eq!(decl("", ""), [
        //     "",
        //     "",
        // ]);
    }

    #[test]
    fn circled_digits() {
        // Circled one, ①
        assert_eq!(decl("жва́ло", "с 1a(1)"), [
            "жва́ло, жва́ла, жва́лу, жва́ло, жва́лом, жва́ле",
            "жва́лы, жва́л, жва́лам, жва́лы, жва́лами, жва́лах",
        ]);
        assert_eq!(decl("го́рлышко", "с 3*a(1)"), [
            "го́рлышко, го́рлышка, го́рлышку, го́рлышко, го́рлышком, го́рлышке",
            "го́рлышки, го́рлышек, го́рлышкам, го́рлышки, го́рлышками, го́рлышках",
        ]);
        assert_eq!(decl("ве́тер", "м 1*c(1)"), [
            "ве́тер, ве́тра, ве́тру, ве́тер, ве́тром, ве́тре",
            "ветра́, ветро́в, ветра́м, ветра́, ветра́ми, ветра́х",
        ]);
        assert_eq!(decl("кра́й", "м 6c(1)"), [
            "кра́й, кра́я, кра́ю, кра́й, кра́ем, кра́е",
            "края́, краё́в, края́м, края́, края́ми, края́х",
        ]);

        // Circled two, ②
        assert_eq!(decl("о́блако", "с 3c(2)"), [
            "о́блако, о́блака, о́блаку, о́блако, о́блаком, о́блаке",
            "облака́, облако́в, облака́м, облака́, облака́ми, облака́х",
        ]);
        assert_eq!(decl("око́нце", "с 5*a(2)"), [
            "око́нце, око́нца, око́нцу, око́нце, око́нцем, око́нце",
            "око́нца, око́нцев, око́нцам, око́нца, око́нцами, око́нцах",
        ]);
        assert_eq!(decl("мясцо́", "с 5*b(2)"), [
            "мясцо́, мясца́, мясцу́, мясцо́, мясцо́м, мясце́",
            "мясца́, мясцо́в, мясца́м, мясца́, мясца́ми, мясца́х",
        ]);
        assert_eq!(decl("подпо́лье", "с 6*a(2)"), [
            "подпо́лье, подпо́лья, подпо́лью, подпо́лье, подпо́льем, подпо́лье",
            "подпо́лья, подпо́льев, подпо́льям, подпо́лья, подпо́льями, подпо́льях",
        ]);
        assert_eq!(decl("жнивьё́", "с 6*b(2)"), [
            "жнивьё́, жнивья́, жнивью́, жнивьё́, жнивьё́м, жнивье́",
            "жнивья́, жнивьё́в, жнивья́м, жнивья́, жнивья́ми, жнивья́х",
        ]);
        assert_eq!(decl("ба́йт", "м 1a(2)"), [
            "ба́йт, ба́йта, ба́йту, ба́йт, ба́йтом, ба́йте",
            "ба́йты, ба́йт, ба́йтам, ба́йты, ба́йтами, ба́йтах",
        ]);
        // Note: There are no masculine nouns with ②, of type 2/6/7/8?
        assert_eq!(decl("ноздря́", "ж 2f(2)"), [
            "ноздря́, ноздри́, ноздре́, ноздрю́, ноздрё́й, ноздре́",
            "но́здри, ноздре́й, ноздря́м, но́здри, ноздря́ми, ноздря́х",
        ]);

        // Circled one and two, ①②
        assert_eq!(decl("гла́з", "м 1c(1)(2)"), [
            "гла́з, гла́за, гла́зу, гла́з, гла́зом, гла́зе",
            "глаза́, гла́з, глаза́м, глаза́, глаза́ми, глаза́х",
        ]);
        assert_eq!(decl("очко́", "с 3*b(1)(2)"), [
            "очко́, очка́, очку́, очко́, очко́м, очке́",
            "очки́, очко́в, очка́м, очки́, очка́ми, очка́х",
        ]);

        // Circled three, ③
        assert_eq!(decl("чи́й", "м 7a(3)"), [
            "чи́й, чи́я, чи́ю, чи́й, чи́ем, чи́е",
            "чи́и, чи́ев, чи́ям, чи́и, чи́ями, чи́ях",
        ]);
        assert_eq!(decl("ли́я", "жо 7a(3)"), [
            "ли́я, ли́и, ли́е, ли́ю, ли́ей, ли́е",
            "ли́и, ли́й, ли́ям, ли́й, ли́ями, ли́ях",
        ]);

        // assert_eq!(decl("", ""), [
        //     "",
        //     "",
        // ]);
    }
}
