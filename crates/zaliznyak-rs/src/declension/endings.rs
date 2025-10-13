use crate::{
    categories::{DeclInfo, Gender, IntoAnimacy, IntoNumber},
    declension::{
        AdjectiveDeclension, NounDeclension, NounStemType, PronounDeclension, PronounStemType,
        endings_tables::{ADJECTIVE_LOOKUP, Endings, NOUN_LOOKUP, PRONOUN_LOOKUP},
    },
    word::Utf8Letter,
};

impl NounDeclension {
    /// Returns a noun ending according to this declension and info.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::{declension::NounDeclension, word::Utf8LetterSlice};
    ///
    /// // Example word: я́блоко
    /// let decl: NounDeclension = "3a(1)".parse().unwrap();
    ///
    /// // я́блок-о, я́блок-а, я́блок-у
    /// assert_eq!(decl.find_ending("И.п. ед.ч. с.р.".parse().unwrap()).as_str(), "о");
    /// assert_eq!(decl.find_ending("Р.п. ед.ч. с.р.".parse().unwrap()).as_str(), "а");
    /// assert_eq!(decl.find_ending("Д.п. ед.ч. с.р.".parse().unwrap()).as_str(), "у");
    ///
    /// // я́блок-и, я́блок, я́блок-ам
    /// assert_eq!(decl.find_ending("И.п. мн.ч. с.р.".parse().unwrap()).as_str(), "и");
    /// assert_eq!(decl.find_ending("Р.п. мн.ч. с.р.".parse().unwrap()).as_str(), "");
    /// assert_eq!(decl.find_ending("Д.п. мн.ч. с.р.".parse().unwrap()).as_str(), "ам");
    /// ```
    pub const fn find_ending(mut self, mut info: DeclInfo) -> &'static [Utf8Letter] {
        if self.flags.has_any_circled_digits() {
            if info.is_plural() {
                let is_gen = info.case.acc_is_gen(info);

                // Plural, Nominative case, overridden by (1)
                // Plural, Genitive case, overridden by (2)
                if is_gen == Some(false) && self.flags.has_circled_one()
                    || is_gen == Some(true) && self.flags.has_circled_two()
                {
                    info.gender = match info.gender {
                        Gender::Masculine => Gender::Neuter,
                        _ => Gender::Masculine,
                    };
                }
            } else {
                // Singular, stem type 7, overridden by (3)
                if self.flags.has_circled_three() {
                    // Note(hack): Stem type can be set to 6, to achieve 'е' endings
                    //   in Prepositional Feminine/Neuter and Dative Feminine forms.
                    self.stem_type = NounStemType::Type6;
                }
            }
        }

        // Find un-stressed and stressed ending indices
        let endings = self.lookup_endings(info);

        // Check if stress affects the choice of the ending, and return appropriate ending
        let is_stressed = endings.invariant() || self.stress.is_ending_stressed(info);

        endings.get(is_stressed)
    }

    const fn lookup_endings(self, info: DeclInfo) -> Endings {
        // [case:6] [number:2] [gender:3] [stem:8] = [total:288]
        let mut index = info.case as usize;
        index = index * 2 + info.number as usize;
        index = index * 3 + info.gender as usize;
        index = index * 8 + self.stem_type as usize;

        let mut endings = unsafe { *NOUN_LOOKUP.get_unchecked(index) };

        // Check if the ending depends on animacy (accusative case)
        if endings.is_acc() {
            // Adjust index for new case (acc -> nom/gen)
            index -= (info.case as usize - info.animacy.acc_case() as usize) * (2 * 3 * 8);
            endings = unsafe { *NOUN_LOOKUP.get_unchecked(index) };
        }
        endings
    }
}

impl PronounDeclension {
    /// Returns a pronoun ending according to this declension and info.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::{declension::PronounDeclension, word::Utf8LetterSlice};
    ///
    /// // Example: госпо́день п <мс 2*a>
    /// let decl: PronounDeclension = "2*a".parse().unwrap();
    ///
    /// // госпо́ден-ь, госпо́дн-я, госпо́дн-его, госпо́ден-ь
    /// assert_eq!(decl.find_ending("И.п. ед.ч. м.р.".parse().unwrap()).as_str(), "ь");
    /// assert_eq!(decl.find_ending("Р.п. ед.ч. м.р.".parse().unwrap()).as_str(), "я");
    /// assert_eq!(decl.find_ending("В.п. ед.ч. м.р. одуш.".parse().unwrap()).as_str(), "его");
    /// assert_eq!(decl.find_ending("В.п. ед.ч. м.р. неод.".parse().unwrap()).as_str(), "ь");
    ///
    /// // госпо́дн-и, госпо́дн-их, госпо́дн-им
    /// assert_eq!(decl.find_ending("И.п. мн.ч.".parse().unwrap()).as_str(), "и");
    /// assert_eq!(decl.find_ending("Р.п. мн.ч.".parse().unwrap()).as_str(), "их");
    /// assert_eq!(decl.find_ending("Д.п. мн.ч.".parse().unwrap()).as_str(), "им");
    /// ```
    pub const fn find_ending(self, info: DeclInfo) -> &'static [Utf8Letter] {
        // Find un-stressed and stressed ending indices
        let endings = self.lookup_endings(info);

        // Check if stress affects the choice of the ending, and return appropriate ending
        let stressed = endings.invariant() || self.stress.is_ending_stressed(info);

        endings.get(stressed)
    }

    const fn lookup_endings(self, info: DeclInfo) -> Endings {
        // [case:6] [gender|plural:4] [stem:4] = [total:96]
        let mut index = info.case as usize;
        index = index * 4 + (if info.is_singular() { info.gender as usize } else { 3 });
        index = index * 4 + self.stem_type as usize;

        let mut endings = unsafe { *PRONOUN_LOOKUP.get_unchecked(index) };

        // Check if the ending depends on animacy (accusative case)
        if endings.is_acc() {
            // Stem type 2 pronouns' accusative case (animate, specifically) is not consistent.
            // Normally, the endings of Genitive of the same stem type are used, but those of
            // type 2 are "short forms", while accusative animate still uses the full forms,
            // characteristic of type 4. Notably, though, accusative inanimate is unaffected.
            //
            // Example: господень п <мс 2*a>: GEN господня, ACC AN господнего, ACC INAN господень.
            if self.stem_type == PronounStemType::Type2 && info.is_animate() {
                index += PronounStemType::Type4 as usize - PronounStemType::Type2 as usize;
            }

            // Adjust index for new case (acc -> nom/gen)
            index -= (info.case as usize - info.animacy.acc_case() as usize) * (4 * 4);
            endings = unsafe { *PRONOUN_LOOKUP.get_unchecked(index) };
        }
        endings
    }
}

impl AdjectiveDeclension {
    /// Returns an adjective full form ending according to this declension and info.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::{declension::AdjectiveDeclension, word::Utf8LetterSlice};
    ///
    /// // Example word: кра́сный п 1*a/c″
    /// let decl: AdjectiveDeclension = "1*a/c''".parse().unwrap();
    ///
    /// // кра́сн-ый, кра́сн-ое, кра́сн-ая, кра́сн-ые
    /// assert_eq!(decl.find_ending("И.п. ед.ч. м.р.".parse().unwrap()).as_str(), "ый");
    /// assert_eq!(decl.find_ending("И.п. ед.ч. с.р.".parse().unwrap()).as_str(), "ое");
    /// assert_eq!(decl.find_ending("И.п. ед.ч. ж.р.".parse().unwrap()).as_str(), "ая");
    /// assert_eq!(decl.find_ending("И.п. мн.ч.".parse().unwrap()).as_str(), "ые");
    ///
    /// // кра́сн-ого, кра́сн-ого, кра́сн-ой, кра́сн-ых
    /// assert_eq!(decl.find_ending("Р.п. ед.ч. м.р.".parse().unwrap()).as_str(), "ого");
    /// assert_eq!(decl.find_ending("Р.п. ед.ч. с.р.".parse().unwrap()).as_str(), "ого");
    /// assert_eq!(decl.find_ending("Р.п. ед.ч. ж.р.".parse().unwrap()).as_str(), "ой");
    /// assert_eq!(decl.find_ending("Р.п. мн.ч.".parse().unwrap()).as_str(), "ых");
    /// ```
    pub const fn find_ending(self, info: DeclInfo) -> &'static [Utf8Letter] {
        // Find un-stressed and stressed ending indices
        let endings = self.lookup_endings(info, info.case as u8);

        // Check if stress affects the choice of the ending, and return appropriate ending
        let stressed = endings.invariant() || self.stress.full.is_ending_stressed();

        endings.get(stressed)
    }

    /// Returns an adjective short form ending according to this declension and info.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::{declension::AdjectiveDeclension, word::Utf8LetterSlice};
    ///
    /// // Example word: кра́сный п 1*a/c″
    /// let decl: AdjectiveDeclension = "1*a/c''".parse().unwrap();
    ///
    /// // кра́сен, кра́сн-о, красн-а́, красн-ы́
    /// assert_eq!(decl.find_ending_short("ед.ч. м.р.".parse().unwrap()).as_str(), "");
    /// assert_eq!(decl.find_ending_short("ед.ч. с.р.".parse().unwrap()).as_str(), "о");
    /// assert_eq!(decl.find_ending_short("ед.ч. ж.р.".parse().unwrap()).as_str(), "а");
    /// assert_eq!(decl.find_ending_short("мн.ч.".parse().unwrap()).as_str(), "ы");
    /// ```
    pub const fn find_ending_short(self, info: DeclInfo) -> &'static [Utf8Letter] {
        // Find un-stressed and stressed ending indices
        let endings = self.lookup_endings(info, 6);

        // Note: in ambiguous scenarios (None value) endings are assumed to be stressed,
        //   since it doesn't look like there are any adjectives that vary like this anyway.
        let stressed = endings.invariant()
            || self.stress.short.is_ending_stressed(info.number, info.gender).unwrap_or(true);

        endings.get(stressed)
    }

    const fn lookup_endings(self, info: DeclInfo, case_form: u8) -> Endings {
        // [case+short form:7] [gender|plural:4] [stem_type:6] = [total:168]
        let mut index = case_form as usize;
        index = index * 4 + (if info.is_singular() { info.gender as usize } else { 3 });
        index = index * 6 + self.stem_type as usize;

        let mut endings = unsafe { *ADJECTIVE_LOOKUP.get_unchecked(index) };

        // Check if the ending depends on animacy (accusative case)
        if endings.is_acc() && case_form != 6 {
            // Adjust index for new case (acc -> nom/gen)
            index -= (info.case as usize - info.animacy.acc_case() as usize) * (4 * 6);
            endings = unsafe { *ADJECTIVE_LOOKUP.get_unchecked(index) };
        }
        endings
    }
}
