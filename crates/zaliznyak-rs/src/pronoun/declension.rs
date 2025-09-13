use crate::{
    categories::{DeclInfo, Gender, IntoNumber},
    declension::{Declension, PronounDeclension},
    pronoun::{Pronoun, PronounInfo},
    util::InflectionBuf,
    word::{Utf8Letter, Word, WordBuf},
};

impl Pronoun {
    pub fn inflect(&self, info: DeclInfo) -> WordBuf {
        self.info.inflect(&self.stem, info)
    }

    pub fn inflect_into<'a>(&self, info: DeclInfo, dst: &'a mut [Utf8Letter]) -> Word<'a> {
        self.info.inflect_into(&self.stem, info, dst)
    }
}

impl PronounInfo {
    pub fn inflect(&self, stem: &str, info: DeclInfo) -> WordBuf {
        let mut buf = WordBuf::with_capacity_for(stem);
        buf.inflect(|dst| self.inflect_into(stem, info, dst));
        buf
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
                Declension::Pronoun(decl) => decl.inflect(info, &mut buf),
                Declension::Adjective(decl) => decl.inflect(info, &mut buf),
                Declension::Noun(_) => unimplemented!(), // Pronouns don't decline by noun declension
            };
        }

        buf.into()
    }
}

impl PronounDeclension {
    pub(crate) fn inflect(self, info: DeclInfo, buf: &mut InflectionBuf) {
        buf.append_to_ending(self.find_ending(info));

        if self.flags.has_star() {
            self.apply_vowel_alternation(info, buf);
        }
    }

    fn apply_vowel_alternation(self, info: DeclInfo, buf: &mut InflectionBuf) {
        // Vowel alternation type A
        // Singular masculine nominative form is unchanged
        if info.is_singular()
            && info.gender == Gender::Masculine
            && info.case.is_nom_or_acc_inan(info)
        {
            return;
        }

        // Find the alternating LAST vowel
        let Some(found) = buf.stem_mut().iter_mut().enumerate().rfind(|x| x.1.is_vowel()) else {
            todo!("Handle absence of vowels in the stem?")
        };
        let (vowel_index, vowel) = found;

        // Extend vowel's lifetime, to allow accessing stem() and then setting vowel
        let vowel = unsafe { std::mem::transmute::<&mut Utf8Letter, &mut Utf8Letter>(vowel) };

        match vowel {
            Utf8Letter::О => {
                // 'о' is simply removed
                buf.remove_stem_char_at(vowel_index);
            },
            Utf8Letter::И => {
                // Replace 'и' with 'ь' (pronoun stem 6 only alternation)
                *vowel = Utf8Letter::Ь;
            },
            Utf8Letter::Е | Utf8Letter::Ё => {
                let preceding = buf.stem().get(vowel_index - 1).copied();

                if preceding.is_some_and(|x| x.is_vowel()) {
                    // 1) is replaced with 'й' when after a vowel
                    *vowel = Utf8Letter::Й;
                } else if preceding == Some(Utf8Letter::Л) {
                    // 2)c) is replaced with 'ь', when after 'л'
                    *vowel = Utf8Letter::Ь;
                } else {
                    // 3) removed in all other cases
                    buf.remove_stem_char_at(vowel_index);
                }
            },
            _ => {
                todo!("Handle invalid vowel alternation")
            },
        };
    }
}
