use crate::categories::{
    Animacy, Case, CaseEx, Gender, GenderEx, Number, Person, Tense,
    traits::{IntoAnimacy, IntoCaseEx, IntoGenderEx, IntoNumber, IntoTense},
};

impl CaseEx {
    pub const fn abbr_upper(self) -> &'static str {
        match self {
            Self::Nominative => "NOM",
            Self::Genitive => "GEN",
            Self::Dative => "DAT",
            Self::Accusative => "ACC",
            Self::Instrumental => "INS",
            Self::Prepositional => "PRP",
            Self::Partitive => "PRT",
            Self::Translative => "TRANSL",
            Self::Locative => "LOC",
        }
    }
    pub const fn abbr_lower(self) -> &'static str {
        match self {
            Self::Nominative => "nom",
            Self::Genitive => "gen",
            Self::Dative => "dat",
            Self::Accusative => "acc",
            Self::Instrumental => "ins",
            Self::Prepositional => "prp",
            Self::Partitive => "prt",
            Self::Translative => "transl",
            Self::Locative => "loc",
        }
    }
    pub const fn abbr_smcp(self) -> &'static str {
        // Note: small caps 'ꜱ' (U+A731) may not render correctly in some fonts,
        //       so a regular 's' can be used instead for better consistency.
        match self {
            Self::Nominative => "ɴᴏᴍ",
            Self::Genitive => "ɢᴇɴ",
            Self::Dative => "ᴅᴀᴛ",
            Self::Accusative => "ᴀᴄᴄ",
            Self::Instrumental => "ɪɴꜱ",
            Self::Prepositional => "ᴘʀᴘ",
            Self::Partitive => "ᴘʀᴛ",
            Self::Translative => "ᴛʀᴀɴꜱʟ",
            Self::Locative => "ʟᴏᴄ",
        }
    }
}
impl Case {
    pub const fn abbr_upper(self) -> &'static str {
        self.case_ex().abbr_upper()
    }
    pub const fn abbr_lower(self) -> &'static str {
        self.case_ex().abbr_lower()
    }
    pub const fn abbr_smcp(self) -> &'static str {
        self.case_ex().abbr_smcp()
    }
}

impl GenderEx {
    pub const fn abbr_upper(self) -> &'static str {
        match self {
            Self::Masculine => "MASC",
            Self::Neuter => "NEUT",
            Self::Feminine => "FEM",
            Self::Common => "MASC/FEM",
        }
    }
    pub const fn abbr_lower(self) -> &'static str {
        match self {
            Self::Masculine => "masc",
            Self::Neuter => "neut",
            Self::Feminine => "fem",
            Self::Common => "masc/fem",
        }
    }
    pub const fn abbr_smcp(self) -> &'static str {
        // Note: small caps 'ꜰ' (U+A730) may not render correctly in some fonts.
        // Note: small caps 'ꜱ' (U+A731) may not render correctly in some fonts,
        //       so a regular 's' can be used instead for better consistency.
        match self {
            Self::Masculine => "ᴍᴀꜱᴄ",
            Self::Neuter => "ɴᴇᴜᴛ",
            Self::Feminine => "ꜰᴇᴍ",
            Self::Common => "ᴍᴀꜱᴄ/ꜰᴇᴍ",
        }
    }
}
impl Gender {
    pub const fn abbr_upper(self) -> &'static str {
        self.gender_ex().abbr_upper()
    }
    pub const fn abbr_lower(self) -> &'static str {
        self.gender_ex().abbr_lower()
    }
    pub const fn abbr_smcp(self) -> &'static str {
        self.gender_ex().abbr_smcp()
    }
}

impl Animacy {
    pub const fn abbr_upper(self) -> &'static str {
        if self.is_inanimate() { "INAN" } else { "AN" }
    }
    pub const fn abbr_lower(self) -> &'static str {
        if self.is_inanimate() { "inan" } else { "an" }
    }
    pub const fn abbr_smcp(self) -> &'static str {
        if self.is_inanimate() { "ɪɴᴀɴ" } else { "ᴀɴ" }
    }
}
impl Number {
    pub const fn abbr_upper(self) -> &'static str {
        if self.is_singular() { "SG" } else { "PL" }
    }
    pub const fn abbr_lower(self) -> &'static str {
        if self.is_singular() { "sg" } else { "pl" }
    }
    pub const fn abbr_smcp(self) -> &'static str {
        // Note: small caps 'ꜱ' (U+A731) may not render correctly in some fonts,
        //       so a regular 's' can be used instead for better consistency.
        if self.is_singular() { "ꜱɢ" } else { "ᴘʟ" }
    }
}

impl Tense {
    pub const fn abbr_upper(self) -> &'static str {
        if self.is_present() { "PRS" } else { "PST" }
    }
    pub const fn abbr_lower(self) -> &'static str {
        if self.is_present() { "prs" } else { "pst" }
    }
    pub const fn abbr_smcp(self) -> &'static str {
        // Note: small caps 'ꜱ' (U+A731) may not render correctly in some fonts,
        //       so a regular 's' can be used instead for better consistency.
        if self.is_present() { "ᴘʀꜱ" } else { "ᴘꜱᴛ" }
    }
}
impl Person {
    pub const fn to_digit(self) -> u8 {
        match self {
            Self::First => 1,
            Self::Second => 2,
            Self::Third => 3,
        }
    }
    pub const fn to_ascii_digit(self) -> u8 {
        b'0' + self.to_digit()
    }
}

macro_rules! abbr_display_impls {
    ($($t:ty),+ $(,)?) => ($(
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.abbr_upper().fmt(f)
            }
        }
    )+);
}
abbr_display_impls! { CaseEx, Case, GenderEx, Gender, Animacy, Number, Tense }

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ascii_digit = self.to_ascii_digit();
        let slice = std::slice::from_ref(&ascii_digit);
        unsafe { str::from_utf8_unchecked(slice) }.fmt(f)
    }
}
