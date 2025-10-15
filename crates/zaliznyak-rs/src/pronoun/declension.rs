use crate::{
    categories::{DeclInfo, Gender, IntoNumber},
    declension::{Declension, PronounDeclension},
    pronoun::{Pronoun, PronounInfo},
    util::InflectionBuf,
    word::{Utf8Letter, Utf8LetterSlice, Word, WordBuf},
};

impl Pronoun {
    pub fn inflect(&self, info: DeclInfo) -> WordBuf {
        self.info.inflect(self.stem.borrow(), info)
    }
}

impl PronounInfo {
    pub fn inflect(&self, stem: Word, info: DeclInfo) -> WordBuf {
        let mut word = WordBuf::with_stem(stem, 5);
        let mut buf = InflectionBuf::new(&mut word);

        if let Some(decl) = self.declension {
            match decl {
                Declension::Pronoun(decl) => decl.inflect(info, &mut buf),
                Declension::Adjective(decl) => decl.inflect(info, &mut buf),
                Declension::Noun(_) => unimplemented!(), // Pronouns don't decline by noun declension
            };
        }

        buf.finish(&mut word);
        word
    }
}

impl PronounDeclension {
    pub(crate) fn inflect(self, info: DeclInfo, buf: &mut InflectionBuf) {
        // Determine the stress position
        buf.stress = self.stress.pos(info);

        // Append the standard ending
        buf.append_to_ending(self.find_ending(info).as_str());

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

        // SAFETY: The InflectionBuf isn't modified between here and the assignment of vowel.
        let vowel = unsafe { &mut *&raw mut *vowel };

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
