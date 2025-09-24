use bitflags::bitflags;

bitflags! {
    /// A set of declension flags.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::declension::DeclensionFlags;
    ///
    /// let flags: DeclensionFlags = "*①".parse().unwrap();
    /// assert_eq!(flags, DeclensionFlags::STAR | DeclensionFlags::CIRCLED_ONE);
    ///
    /// let flags: DeclensionFlags = "(2), ё".parse().unwrap();
    /// assert_eq!(flags, DeclensionFlags::CIRCLED_TWO | DeclensionFlags::ALTERNATING_YO);
    /// ```
    #[derive(Debug, Copy, Eq)]
    #[derive_const(Clone)]
    pub struct DeclensionFlags: u8 {
        /// The ° flag, indicating unique stem alternation. Used only by nouns.
        /// [See the dictionary for more details](https://gramdict.ru/declension/symbols#superscript-circle).
        const CIRCLE = 1 << 0;
        /// The * flag, indicating vowel alteration in the stem.
        /// [See the dictionary for more details](https://gramdict.ru/declension/symbols#star).
        const STAR = 1 << 1;
        /// The ① flag, indicating, in nouns: an ending of a different gender in nominative plural form;
        /// in adjectives: loss of last 'н' in masculine short form.
        /// [See the dictionary for more details](https://gramdict.ru/declension/symbols#circled-number).
        const CIRCLED_ONE = 1 << 2;
        /// The ② flag, indicating, in nouns: an ending of a different gender in genitive plural form;
        /// in adjectives: loss of last 'н' in all short forms.
        /// [See the dictionary for more details](https://gramdict.ru/declension/symbols#circled-number).
        const CIRCLED_TWO = 1 << 3;
        /// The ③ flag, indicating in stem type 7 nouns, that in prepositional singular (all genders) and
        /// dative singular feminine, an ending of 'е' is also acceptable and even more prevalent than 'и'.
        /// [See the dictionary for more details](https://gramdict.ru/declension/symbols#circled-number).
        const CIRCLED_THREE = 1 << 4;
        /// The ё flag, indicating a 'е'/'ё' alternation in the stem.
        /// [See the dictionary for more details](https://gramdict.ru/declension/symbols#yo).
        const ALTERNATING_YO = 1 << 5;
    }
}

impl DeclensionFlags {
    /// Returns `true` if this contains the ° flag.
    pub const fn has_circle(self) -> bool {
        self.intersects(Self::CIRCLE)
    }
    /// Returns `true` if this contains the * flag.
    pub const fn has_star(self) -> bool {
        self.intersects(Self::STAR)
    }
    /// Returns `true` if this contains the ① flag.
    pub const fn has_circled_one(self) -> bool {
        self.intersects(Self::CIRCLED_ONE)
    }
    /// Returns `true` if this contains the ② flag.
    pub const fn has_circled_two(self) -> bool {
        self.intersects(Self::CIRCLED_TWO)
    }
    /// Returns `true` if this contains the ③ flag.
    pub const fn has_circled_three(self) -> bool {
        self.intersects(Self::CIRCLED_THREE)
    }
    /// Returns `true` if this contains the ё flag.
    pub const fn has_alternating_yo(self) -> bool {
        self.intersects(Self::ALTERNATING_YO)
    }

    const LEADING: Self = Self::CIRCLE.union(Self::STAR);
    const DIGITS: Self = Self::CIRCLED_ONE.union(Self::CIRCLED_TWO).union(Self::CIRCLED_THREE);
    const TRAILING: Self = Self::DIGITS.union(Self::ALTERNATING_YO);

    /// Returns `true` if this contains any leading flags: ° or *.
    pub const fn has_any_leading_flags(self) -> bool {
        self.intersects(Self::LEADING)
    }
    /// Returns `true` if this contains any circled digit flags: ①, ② or ③.
    pub const fn has_any_circled_digits(self) -> bool {
        self.intersects(Self::DIGITS)
    }
    /// Returns `true` if this contains any trailing flags: ①, ②, ③ or ё.
    pub const fn has_any_trailing_flags(self) -> bool {
        self.intersects(Self::TRAILING)
    }

    /// Returns the corresponding circled digit.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::declension::DeclensionFlags;
    ///
    /// assert_eq!(DeclensionFlags::circled_digit(0), None);
    /// assert_eq!(DeclensionFlags::circled_digit(1), Some(DeclensionFlags::CIRCLED_ONE));
    /// assert_eq!(DeclensionFlags::circled_digit(2), Some(DeclensionFlags::CIRCLED_TWO));
    /// assert_eq!(DeclensionFlags::circled_digit(3), Some(DeclensionFlags::CIRCLED_THREE));
    /// assert_eq!(DeclensionFlags::circled_digit(4), None);
    /// ```
    pub const fn circled_digit(digit: u8) -> Option<Self> {
        const ZERO_BITS: u8 = DeclensionFlags::CIRCLED_ONE.bits() >> 1;

        if matches!(digit, 1..=3) {
            Some(Self::from_bits_retain(ZERO_BITS << digit))
        } else {
            None
        }
    }
}

// FIXME(const-hack): Replace these with #[derive_const], once bitflags crate supports it.
impl const Default for DeclensionFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl const PartialEq for DeclensionFlags {
    fn eq(&self, other: &Self) -> bool {
        self.bits() == other.bits()
    }
}
impl std::hash::Hash for DeclensionFlags {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(self.bits());
    }
}
