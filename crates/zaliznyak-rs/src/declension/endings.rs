use crate::{
    categories::{DeclInfo, IntoNumber},
    declension::{
        AdjectiveDeclension, NounDeclension, PronounDeclension, PronounStemType,
        endings_tables::{ADJECTIVE_LOOKUP, NOUN_LOOKUP, PRONOUN_LOOKUP, get_ending_by_index},
    },
};

impl NounDeclension {
    pub const fn find_ending(self, info: DeclInfo) -> &'static str {
        // Find un-stressed and stressed ending indices
        let (un_str, str) = Self::lookup_ending_indices(info, self.stem_type as u8);

        // Check if stress affects the choice of the ending, and return appropriate ending
        let is_stressed = un_str == str || self.stress.is_ending_stressed(info);
        get_ending_by_index(if is_stressed { str } else { un_str })
    }

    const fn lookup_ending_indices(info: DeclInfo, stem_type: u8) -> (u8, u8) {
        // [case:6] [number:2] [gender:3] [stem:8] = [total:288]
        let mut index = info.case as usize;
        index = index * 2 + info.number as usize;
        index = index * 3 + info.gender as usize;
        index = index * 8 + stem_type as usize;

        let mut indices = *unsafe { NOUN_LOOKUP.get_unchecked(index) };

        // 0 means that the ending depends on animacy (accusative case)
        if indices.0 == 0 {
            // Adjust index for new case (acc -> nom/gen)
            index -= (info.case as usize - info.animacy.acc_case() as usize) * (2 * 3 * 8);
            indices = *unsafe { NOUN_LOOKUP.get_unchecked(index) };
        }
        indices
    }
}

impl PronounDeclension {
    pub const fn find_ending(self, info: DeclInfo) -> &'static str {
        // Find un-stressed and stressed ending indices
        let (un_str, str) = Self::lookup_ending_indices(info, self.stem_type as u8);

        // Check if stress affects the choice of the ending, and return appropriate ending
        let stressed = un_str == str || self.stress.is_ending_stressed(info);
        get_ending_by_index(if stressed { str } else { un_str })
    }

    const fn lookup_ending_indices(info: DeclInfo, stem_type: u8) -> (u8, u8) {
        // [case:6] [gender|plural:4] [stem:4] = [total:96]
        let mut index = info.case as usize;
        index = index * 4 + (if info.is_singular() { info.gender as usize } else { 3 });
        index = index * 4 + stem_type as usize;

        let mut indices = *unsafe { PRONOUN_LOOKUP.get_unchecked(index) };

        // 0 means that the ending depends on animacy (accusative case)
        if indices.0 == 0 {
            // Stem type 2 pronouns' accusative case is not consistent. Normally, the endings
            // of either Nominative or Genitive of the same stem type are used, but those of
            // type 2 are "shortened", while Accusative still uses full form of those.
            // Example: господень <мс 2>: GEN господня, but ACC господнего.
            if stem_type == PronounStemType::Type2 as u8 {
                index -= (PronounStemType::Type4 as u8 - PronounStemType::Type2 as u8) as usize;
            }

            // Adjust index for new case (acc -> nom/gen)
            index -= (info.case as usize - info.animacy.acc_case() as usize) * (4 * 4);
            indices = *unsafe { PRONOUN_LOOKUP.get_unchecked(index) };
        }
        indices
    }
}

impl AdjectiveDeclension {
    pub const fn find_ending(self, info: DeclInfo) -> &'static str {
        // Find un-stressed and stressed ending indices
        let (un_str, str) = Self::lookup_ending_indices(info, self.stem_type as u8);

        // Check if stress affects the choice of the ending, and return appropriate ending
        let stressed = un_str == str || self.stress.full.is_ending_stressed();
        get_ending_by_index(if stressed { str } else { un_str })
    }

    pub const fn find_ending_short_form(self, info: DeclInfo) -> &'static str {
        // Find un-stressed and stressed ending indices
        let (un_str, str) = Self::lookup_ending_indices(info, 7);

        // Note: in ambiguous scenarios (None value) endings are assumed to be stressed,
        // since it doesn't look like there are any adjectives that vary like this anyway.
        let stressed = un_str == str
            || self.stress.short.is_ending_stressed(info.number, info.gender).unwrap_or(true);

        get_ending_by_index(if stressed { str } else { un_str })
    }

    const fn lookup_ending_indices(info: DeclInfo, stem_type: u8) -> (u8, u8) {
        // [case+short form:7] [gender|plural:4] [stem_type:7] = [total:196]
        let mut index = info.case as usize;
        index = index * 4 + (if info.is_singular() { info.gender as usize } else { 3 });
        index = index * 7 + stem_type as usize;

        let mut indices = *unsafe { ADJECTIVE_LOOKUP.get_unchecked(index) };

        // 0 means that the ending depends on animacy (accusative case)
        if indices.0 == 0 {
            // Adjust index for new case (acc -> nom/gen)
            index -= (info.case as usize - info.animacy.acc_case() as usize) * (4 * 7);
            indices = *unsafe { ADJECTIVE_LOOKUP.get_unchecked(index) };
        }
        indices
    }
}
