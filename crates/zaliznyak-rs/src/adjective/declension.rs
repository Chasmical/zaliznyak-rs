use crate::{
    Word, WordBuf,
    adjective::{Adjective, AdjectiveInfo, AdjectiveKind},
    alphabet::Utf8Letter,
    categories::{DeclInfo, Gender, IntoNumber},
    declension::{AdjectiveDeclension, Declension},
    stress::AdjectiveStress,
    util::InflectionBuf,
};

impl Adjective {
    pub fn inflect(&self, info: DeclInfo) -> WordBuf {
        self.info.inflect(&self.stem, info)
    }
    pub fn inflect_short(&self, info: DeclInfo, force: bool) -> Option<WordBuf> {
        self.info.inflect_short(&self.stem, info, force)
    }
    pub fn inflect_comparative(&self) -> Option<WordBuf> {
        self.info.inflect_comparative(&self.stem)
    }

    pub fn inflect_into<'a>(&self, info: DeclInfo, dst: &'a mut [Utf8Letter]) -> Word<'a> {
        self.info.inflect_into(&self.stem, info, dst)
    }
    pub fn inflect_short_into<'a>(
        &self,
        info: DeclInfo,
        force: bool,
        dst: &'a mut [Utf8Letter],
    ) -> Option<Word<'a>> {
        self.info.inflect_short_into(&self.stem, info, force, dst)
    }
    pub fn inflect_comparative_into<'a>(&self, dst: &'a mut [Utf8Letter]) -> Option<Word<'a>> {
        self.info.inflect_comparative_into(&self.stem, dst)
    }
}

impl AdjectiveInfo {
    pub fn inflect(&self, stem: &str, info: DeclInfo) -> WordBuf {
        let buf = WordBuf::with_capacity_for(stem);
        buf.with_buf(|dst| self.inflect_into(stem, info, dst))
    }
    pub fn inflect_short(&self, stem: &str, info: DeclInfo, force: bool) -> Option<WordBuf> {
        let buf = WordBuf::with_capacity_for(stem);
        buf.with_buf_opt(|dst| self.inflect_short_into(stem, info, force, dst))
    }
    pub fn inflect_comparative(&self, stem: &str) -> Option<WordBuf> {
        let buf = WordBuf::with_capacity_for(stem);
        buf.with_buf_opt(|dst| self.inflect_comparative_into(stem, dst))
    }

    pub fn inflect_into<'a>(
        &self,
        stem: &str,
        info: DeclInfo,
        dst: &'a mut [Utf8Letter],
    ) -> Word<'a> {
        let mut buf = InflectionBuf::with_stem_in(stem, dst);

        if let Some(decl) = self.declension {
            match decl {
                Declension::Adjective(decl) => decl.inflect(info, &mut buf),
                Declension::Pronoun(decl) => decl.inflect(info, &mut buf),
                Declension::Noun(_) => unimplemented!(), // Adjectives don't decline by noun declension
            };
        }

        buf.into()
    }
    pub fn inflect_short_into<'a>(
        &self,
        stem: &str,
        info: DeclInfo,
        force: bool,
        dst: &'a mut [Utf8Letter],
    ) -> Option<Word<'a>> {
        // Only regular adjective-declension adjectives can have short forms.
        // Also, check adjective flags (—✕⌧) to see if there are difficulties.

        if self.kind == AdjectiveKind::Regular
            && self.flags.has_short_form(info).unwrap_or(force)
            && let Some(Declension::Adjective(decl)) = self.declension
        {
            let mut buf = InflectionBuf::with_stem_in(stem, dst);
            decl.inflect_short(info, &mut buf);
            Some(buf.into())
        } else {
            None
        }
    }
    pub fn inflect_comparative_into<'a>(
        &self,
        stem: &str,
        dst: &'a mut [Utf8Letter],
    ) -> Option<Word<'a>> {
        // Only regular adjective-declension adjectives can have comparative forms.
        // Also, check adjective flag (~) to see if it has a comparative form.

        if self.kind == AdjectiveKind::Regular
            && !self.flags.has_no_comparative_form()
            && let Some(Declension::Adjective(decl)) = self.declension
        {
            let mut buf = InflectionBuf::with_stem_in(stem, dst);
            decl.inflect_comparative(&mut buf);
            Some(buf.into())
        } else {
            None
        }
    }
}

impl AdjectiveDeclension {
    pub(crate) fn inflect(self, info: DeclInfo, buf: &mut InflectionBuf) {
        buf.append_to_ending(self.find_ending(info));

        if self.flags.has_alternating_yo() {
            self.apply_ye_yo_alternation(buf);
        }
    }

    pub(crate) fn inflect_short(self, info: DeclInfo, buf: &mut InflectionBuf) {
        buf.append_to_ending(self.find_ending_short(info));

        // If declension has (1) or (2), remove the redundant 'н' in short form
        if self.flags.has_circled_two()
            || self.flags.has_circled_one() && info.gender == Gender::Masculine
        {
            buf.shrink_stem_by(1);
        } else if self.flags.has_star() {
            self.apply_vowel_alternation_short(info, buf);
        }

        if self.flags.has_alternating_yo() {
            self.apply_ye_yo_alternation(buf);
        }
    }

    pub(crate) fn inflect_comparative(self, buf: &mut InflectionBuf) {
        // Add one 'е' as the ending
        buf.append_to_ending("е");

        match buf.stem_mut().last_mut() {
            // Replace 'к' with 'ч'
            Some(ch @ Utf8Letter::К) => *ch = Utf8Letter::Ч,
            // Replace 'г' with 'ж'
            Some(ch @ Utf8Letter::Г) => *ch = Utf8Letter::Ж,
            // Replace 'х' with 'ш'
            Some(ch @ Utf8Letter::Х) => *ch = Utf8Letter::Ш,

            _ => {
                // Add another 'е', resulting in 'ее' ending
                buf.append_to_ending("е");

                // Unstress the 'ё' in stem into 'е', since stress always falls on 'ее' ending.
                // (unless the stress is exactly a/a, in which case the stress is on the stem)
                if self.stress != AdjectiveStress::A_A
                    && let Some(yo) = buf.stem_mut().iter_mut().find(|x| **x == Utf8Letter::Ё)
                {
                    *yo = Utf8Letter::Е;
                }
                return;
            },
        };

        // In case of к/г/х, the stress falls on the last stem syllable.
        // If there's a 'ё' in non-last stem vowel position, unstress it into 'е'.
        if let Some(yo) = buf.stem_mut().iter_mut().find(|x| **x == Utf8Letter::Ё) {
            // Extend yo's lifetime, to allow accessing stem() and then setting yo
            let yo = unsafe { std::mem::transmute::<&mut Utf8Letter, &mut Utf8Letter>(yo) };

            let last_vowel = buf.stem().iter().rfind(|x| x.is_vowel()).unwrap();

            // Unstress 'ё' only if it's not the last (stressed) vowel
            if !std::ptr::addr_eq(yo, last_vowel) {
                *yo = Utf8Letter::Е;
            }
        }
    }

    fn apply_ye_yo_alternation(self, buf: &mut InflectionBuf) {
        let (stem, ending) = buf.stem_and_ending_mut();

        // If there's a 'ё' in the stem:
        if let Some(yo) = stem.iter_mut().find(|x| **x == Utf8Letter::Ё) {
            // If stress falls on the ending, unstress 'ё' in the stem into 'е'
            if self.stress.full.is_ending_stressed() && ending.iter().any(|x| x.is_vowel()) {
                *yo = Utf8Letter::Е;
            }
        } else {
            // If there's no 'ё' in the stem, find the 'е' that can be stressed into 'ё'

            // Find the LAST unstressed 'е' in the stem
            let Some(ye) = stem.iter_mut().rfind(|x| **x == Utf8Letter::Е) else {
                todo!("Handle absence of 'е' in the stem?")
            };
            // Extend ye's lifetime, to allow accessing stem() and then setting ye
            let ye = unsafe { std::mem::transmute::<&mut Utf8Letter, &mut Utf8Letter>(ye) };

            let stress_into_yo = {
                if !ending.iter().any(|x| x.is_vowel()) {
                    // If the ending can't receive stress, then stress 'е' in the stem into 'ё'
                    true
                } else {
                    // TODO: check if this 'first vowel' check is relevant for adjectives
                    let first_vowel = buf.stem().iter().find(|x| x.is_vowel());

                    first_vowel.is_some_and(|x| std::ptr::eq(ye, x))
                        && self.stress.full.is_stem_stressed()
                }
            };

            // Stress 'е' in the stem into 'ё'
            if stress_into_yo {
                *ye = Utf8Letter::Ё;
            }
        }
    }

    fn apply_vowel_alternation_short(self, info: DeclInfo, buf: &mut InflectionBuf) {
        use Utf8Letter::*;

        // Vowel alternation type B, only singular masculine form is affected
        if info.is_plural() || info.gender != Gender::Masculine {
            return;
        }

        // At this point, stem type is in range 1..=5 (consonant-ending stems).
        // Stem type 6 adjectives don't have *, and stem type 7 adjectives don't exist.
        // So, it's safe to assume that the last stem char is a consonant.
        let stem = buf.stem_mut();
        let last = stem.last().copied();
        let pre_last = stem.get_mut(stem.len() - 2);

        // 2) if 'ь'/'й' precedes the last consonant, replace 'ь'/'й' with 'ё' or 'е'.
        // E.g. горький (п 3*a/c') - горек; спокойный (п 1*a) - спокоен.
        if let Some(pre_last @ (Ь | Й)) = pre_last {
            // Note: since vowel alternation only affects masculine, and short form stress
            //   doesn't vary in masculine forms, it's okay to unwrap with any default value.
            //   (see AdjectiveShortStress::is_ending_stressed)
            let stressed = last != Some(Ц)
                && self.stress.short.is_ending_stressed(info.number, info.gender).unwrap_or(true);
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
                if last == Some(Ц)
                    || self.stress.short.is_stem_stressed(info.number, info.gender).unwrap_or(true)
                {
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
