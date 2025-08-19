use crate::{
    categories::{Case, DeclInfo, IntoNumber},
    declension::{
        AdjectiveDeclension, NounDeclension, PronounDeclension, PronounStemType,
        endings_tables::{ADJECTIVE_LOOKUP, NOUN_LOOKUP, PRONOUN_LOOKUP, get_ending_by_index},
    },
};

impl NounDeclension {
    pub const fn find_ending(self, info: DeclInfo) -> &'static str {
        let (mut un_str, mut str) = self.ending_lookup(info, info.case);

        if un_str == 0 {
            let case = info.animacy.acc_case();
            (un_str, str) = self.ending_lookup(info, case);
        }

        let is_stressed = un_str == str || self.stress.is_ending_stressed(info);
        get_ending_by_index(if is_stressed { str } else { un_str })
    }

    const fn ending_lookup(self, info: DeclInfo, case: Case) -> (u8, u8) {
        let mut x = case as usize;
        x = x * 2 + info.number as usize;
        x = x * 3 + info.gender as usize;
        x = x * 8 + self.stem_type as usize;
        NOUN_LOOKUP[x]
    }
}

impl PronounDeclension {
    pub const fn find_ending(mut self, info: DeclInfo) -> &'static str {
        let (mut un_str, mut str) = self.ending_lookup(info, info.case);

        if un_str == 0 {
            // Stem type 2 pronouns' accusative case is not consistent. Normally, the endings
            // of either Nominative or Genitive of the same stem type are used, but those of
            // type 2 are "shortened", while Accusative still uses full form of those.
            // Example: господень <мс 2>: GEN господня, but ACC господнего.
            if self.stem_type == PronounStemType::Type2 {
                self.stem_type = PronounStemType::Type4;
            }

            let case = info.animacy.acc_case();
            (un_str, str) = self.ending_lookup(info, case);
        }

        let stressed = un_str == str || self.stress.is_ending_stressed(info);
        get_ending_by_index(if stressed { str } else { un_str })
    }

    const fn ending_lookup(self, info: DeclInfo, case: Case) -> (u8, u8) {
        let mut x = case as usize;
        x = x * 4 + (if info.is_singular() { info.gender as usize } else { 3 });
        x = x * 4 + self.stem_type as usize;
        PRONOUN_LOOKUP[x]
    }
}

impl AdjectiveDeclension {
    pub const fn find_ending(self, info: DeclInfo) -> &'static str {
        let (mut un_str, mut str) = self.ending_lookup(info, info.case);

        if un_str == 0 {
            let case = info.animacy.acc_case();
            (un_str, str) = self.ending_lookup(info, case);
        }

        let stressed = un_str == str || self.stress.full.is_ending_stressed();
        get_ending_by_index(if stressed { str } else { un_str })
    }

    const fn ending_lookup(self, info: DeclInfo, case: Case) -> (u8, u8) {
        let mut x = case as usize;
        x = x * 4 + (if info.is_singular() { info.gender as usize } else { 3 });
        x = x * 7 + self.stem_type as usize;
        ADJECTIVE_LOOKUP[x]
    }
}
