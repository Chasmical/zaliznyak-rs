use crate::categories::{
    Animacy, Case, CaseEx, Gender, GenderEx, Number, Person, Tense,
    traits::{IntoAnimacy, IntoCaseEx, IntoGenderEx, IntoNumber, IntoTense},
};

impl CaseEx {
    /// Abbreviates this case in upper case: NOM, GEN, DAT, ACC, INS, PRP, PRT, TRANSL, LOC.
    #[must_use]
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
    /// Abbreviates this case in lower case: nom, gen, dat, acc, ins, prp, prt, transl, loc.
    #[must_use]
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
    /// Abbreviates this case in small caps: ɴᴏᴍ, ɢᴇɴ, ᴅᴀᴛ, ᴀᴄᴄ, ɪɴꜱ, ᴘʀᴘ, ᴘʀᴛ, ᴛʀᴀɴꜱʟ, ʟᴏᴄ.
    #[must_use]
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
    /// Abbreviates this case in upper case: NOM, GEN, DAT, ACC, INS, PRP.
    #[must_use]
    pub const fn abbr_upper(self) -> &'static str {
        self.case_ex().abbr_upper()
    }
    /// Abbreviates this case in lower case: nom, gen, dat, acc, ins, prp.
    #[must_use]
    pub const fn abbr_lower(self) -> &'static str {
        self.case_ex().abbr_lower()
    }
    /// Abbreviates this case in small caps: ɴᴏᴍ, ɢᴇɴ, ᴅᴀᴛ, ᴀᴄᴄ, ɪɴꜱ, ᴘʀᴘ.
    #[must_use]
    pub const fn abbr_smcp(self) -> &'static str {
        self.case_ex().abbr_smcp()
    }
}

impl GenderEx {
    /// Abbreviates this gender in upper case: MASC, NEUT, FEM, MASC/FEM.
    #[must_use]
    pub const fn abbr_upper(self) -> &'static str {
        match self {
            Self::Masculine => "MASC",
            Self::Neuter => "NEUT",
            Self::Feminine => "FEM",
            Self::Common => "MASC/FEM",
        }
    }
    /// Abbreviates this gender in lower case: masc, neut, fem, masc/fem.
    #[must_use]
    pub const fn abbr_lower(self) -> &'static str {
        match self {
            Self::Masculine => "masc",
            Self::Neuter => "neut",
            Self::Feminine => "fem",
            Self::Common => "masc/fem",
        }
    }
    /// Abbreviates this gender in small caps: ᴍᴀꜱᴄ, ɴᴇᴜᴛ, ꜰᴇᴍ, ᴍᴀꜱᴄ/ꜰᴇᴍ.
    #[must_use]
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
    /// Abbreviates this gender in upper case: MASC, NEUT, FEM.
    #[must_use]
    pub const fn abbr_upper(self) -> &'static str {
        self.gender_ex().abbr_upper()
    }
    /// Abbreviates this gender in lower case: masc, neut, fem.
    #[must_use]
    pub const fn abbr_lower(self) -> &'static str {
        self.gender_ex().abbr_lower()
    }
    /// Abbreviates this gender in small caps: ᴍᴀꜱᴄ, ɴᴇᴜᴛ, ꜰᴇᴍ.
    #[must_use]
    pub const fn abbr_smcp(self) -> &'static str {
        self.gender_ex().abbr_smcp()
    }
}

impl Animacy {
    /// Abbreviates this animacy in upper case: INAN or AN.
    #[must_use]
    pub const fn abbr_upper(self) -> &'static str {
        if self.is_inanimate() { "INAN" } else { "AN" }
    }
    /// Abbreviates this animacy in lower case: inan or an.
    #[must_use]
    pub const fn abbr_lower(self) -> &'static str {
        if self.is_inanimate() { "inan" } else { "an" }
    }
    /// Abbreviates this animacy in small caps: ɪɴᴀɴ or ᴀɴ.
    #[must_use]
    pub const fn abbr_smcp(self) -> &'static str {
        if self.is_inanimate() { "ɪɴᴀɴ" } else { "ᴀɴ" }
    }
}
impl Number {
    /// Abbreviates this number in upper case: SG or PL.
    #[must_use]
    pub const fn abbr_upper(self) -> &'static str {
        if self.is_singular() { "SG" } else { "PL" }
    }
    /// Abbreviates this number in lower case: sg or pl.
    #[must_use]
    pub const fn abbr_lower(self) -> &'static str {
        if self.is_singular() { "sg" } else { "pl" }
    }
    /// Abbreviates this number in small caps: ꜱɢ or ᴘʟ.
    #[must_use]
    pub const fn abbr_smcp(self) -> &'static str {
        // Note: small caps 'ꜱ' (U+A731) may not render correctly in some fonts,
        //       so a regular 's' can be used instead for better consistency.
        if self.is_singular() { "ꜱɢ" } else { "ᴘʟ" }
    }
}

impl Tense {
    /// Abbreviates this tense in upper case: PRS or PST.
    #[must_use]
    pub const fn abbr_upper(self) -> &'static str {
        if self.is_present() { "PRS" } else { "PST" }
    }
    /// Abbreviates this tense in lower case: prs or pst.
    #[must_use]
    pub const fn abbr_lower(self) -> &'static str {
        if self.is_present() { "prs" } else { "pst" }
    }
    /// Abbreviates this tense in small caps: ᴘʀꜱ or ᴘꜱᴛ.
    #[must_use]
    pub const fn abbr_smcp(self) -> &'static str {
        // Note: small caps 'ꜱ' (U+A731) may not render correctly in some fonts,
        //       so a regular 's' can be used instead for better consistency.
        if self.is_present() { "ᴘʀꜱ" } else { "ᴘꜱᴛ" }
    }
}
impl Person {
    /// Converts this person to its corresponding digit.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::categories::Person;
    ///
    /// assert_eq!(Person::First.to_digit(), 1);
    /// assert_eq!(Person::Second.to_digit(), 2);
    /// assert_eq!(Person::Third.to_digit(), 3);
    /// ```
    #[must_use]
    pub const fn to_digit(self) -> u8 {
        match self {
            Self::First => 1,
            Self::Second => 2,
            Self::Third => 3,
        }
    }
    /// Converts this person to its corresponding ASCII digit.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::categories::Person;
    ///
    /// assert_eq!(Person::First.to_ascii_digit(), b'1');
    /// assert_eq!(Person::Second.to_ascii_digit(), b'2');
    /// assert_eq!(Person::Third.to_ascii_digit(), b'3');
    /// ```
    #[must_use]
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
        std::fmt::Write::write_char(f, self.to_ascii_digit() as char)
    }
}
