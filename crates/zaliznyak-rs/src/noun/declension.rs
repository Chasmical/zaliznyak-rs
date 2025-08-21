use crate::{
    alphabet::Utf8Letter,
    categories::DeclInfo,
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
        todo!()
    }

    pub fn apply_vowel_alternation(self, info: DeclInfo, buf: &mut InflectionBuf) {
        todo!()
    }

    pub fn apply_ye_yo_alternation(self, info: DeclInfo, buf: &mut InflectionBuf) {
        todo!()
    }
}
