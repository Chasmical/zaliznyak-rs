use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Eq, Hash)]
    #[derive_const(Clone)]
    pub struct DeclensionFlags: u8 {
        const STAR = 1 << 0;
        const CIRCLE = 1 << 1;
        const CIRCLED_ONE = 1 << 2;
        const CIRCLED_TWO = 1 << 3;
        const CIRCLED_THREE = 1 << 4;
        const ALTERNATING_YO = 1 << 5;
    }
}

impl DeclensionFlags {
    pub const fn has_star(self) -> bool {
        self.intersects(Self::STAR)
    }
    pub const fn has_circle(self) -> bool {
        self.intersects(Self::CIRCLE)
    }
    pub const fn has_circled_one(self) -> bool {
        self.intersects(Self::CIRCLED_ONE)
    }
    pub const fn has_circled_two(self) -> bool {
        self.intersects(Self::CIRCLED_TWO)
    }
    pub const fn has_circled_three(self) -> bool {
        self.intersects(Self::CIRCLED_THREE)
    }
    pub const fn has_alternating_yo(self) -> bool {
        self.intersects(Self::ALTERNATING_YO)
    }

    const LEADING: Self = Self::STAR.union(Self::CIRCLE);
    const DIGITS: Self = Self::CIRCLED_ONE.union(Self::CIRCLED_TWO).union(Self::CIRCLED_THREE);
    const TRAILING: Self = Self::DIGITS.union(Self::ALTERNATING_YO);

    pub const fn has_any_leading_flags(self) -> bool {
        self.intersects(Self::LEADING)
    }
    pub const fn has_any_circled_digits(self) -> bool {
        self.intersects(Self::DIGITS)
    }
    pub const fn has_any_trailing_flags(self) -> bool {
        self.intersects(Self::TRAILING)
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
