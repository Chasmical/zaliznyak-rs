use bitflags::bitflags;

use crate::categories::{DeclInfo, Gender, IntoNumber};

bitflags! {
    // FIXME(const-hack): Derive PartialEq with #[derive_const] when bitflags supports it.
    #[derive(Debug, Copy, PartialEq, Eq, Hash)]
    #[derive_const(Clone)]
    pub struct AdjectiveFlags: u8 {
        const MINUS               = 0b_0001;
        const CROSS               = 0b_0010;
        const BOXED_CROSS         = 0b_0011;
        const NO_COMPARATIVE_FORM = 0b_1000;
    }
}

impl AdjectiveFlags {
    pub const fn has_minus(self) -> bool {
        self.intersects(Self::MINUS)
    }
    pub const fn has_cross(self) -> bool {
        self.intersects(Self::CROSS)
    }
    pub const fn has_no_comparative_form(self) -> bool {
        self.intersects(Self::NO_COMPARATIVE_FORM)
    }

    pub const fn has_short_form(self, info: DeclInfo) -> Option<bool> {
        let flags = self.intersection(Self::BOXED_CROSS);

        if flags.is_empty() {
            // No difficulty with short form
            return Some(true);
        }

        // FIXME(const-hack): Remove `.bits()` calls when ==/>= on bitflags is constified.
        if info.is_singular() && info.gender == Gender::Masculine {
            // ⌧ - none, — and ✕ - difficult
            if flags.bits() == Self::BOXED_CROSS.bits() { Some(false) } else { None }
        } else {
            // — - ok, ✕ and ⌧ - difficult
            if flags.bits() >= Self::CROSS.bits() { None } else { Some(true) }
        }
    }
}
