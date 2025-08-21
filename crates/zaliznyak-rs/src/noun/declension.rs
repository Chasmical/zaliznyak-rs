use crate::{
    alphabet::Utf8Letter,
    categories::{DeclInfo, Gender, IntoNumber},
    declension::{NounDeclension, NounStemType},
    inflection_buf::InflectionBuf,
};

impl NounDeclension {
    pub fn inflect(self, info: DeclInfo, buf: &mut InflectionBuf) {
        buf.append_to_ending(self.find_ending(info));

        if self.flags.has_circle() {
            self.apply_unique_alternation(info, buf);
        }

        if self.stem_type == NounStemType::Type8
            && buf.stem().last().is_some_and(|x| x.is_hissing())
            && let [ya @ Utf8Letter::Я, ..] = buf.ending_mut()
        {
            *ya = Utf8Letter::А;
        }

        if self.flags.has_star() {
            self.apply_vowel_alternation(info, buf);
        }
        if self.flags.has_alternating_yo() {
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
        todo!()
    }

    pub fn apply_ye_yo_alternation(self, info: DeclInfo, buf: &mut InflectionBuf) {
        todo!()
    }
}
