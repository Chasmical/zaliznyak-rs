use crate::{
    stress::{
        AdjectiveFullStress, AdjectiveShortStress, AdjectiveStress, AnyDualStress, AnyStress,
        NounStress, PronounStress, VerbPastStress, VerbPresentStress, VerbStress,
    },
    util::UnsafeBuf,
};

impl AnyStress {
    pub const fn fmt_to(self, dst: &mut [u8; 4]) -> &mut str {
        // Write the letter: a, b, c, d, e, f
        dst[0] = match self.unprime() {
            Self::A => b'a',
            Self::B => b'b',
            Self::C => b'c',
            Self::D => b'd',
            Self::E => b'e',
            Self::F => b'f',
            _ => unreachable!(),
        };

        // If the stress has primes, it will occupy the entire 4-byte buffer
        if self.has_any_primes() {
            // Write UTF-8 bytes of either ′ or ″
            let ch = if self.has_double_prime() { '″' } else { '′' };
            ch.encode_utf8(dst.last_chunk_mut::<3>().unwrap());
            // Return string slice from the entire buffer
            unsafe { str::from_utf8_unchecked_mut(dst) }
        } else {
            // Return string slice of length 1, containing only the letter
            let slice = unsafe { std::slice::from_raw_parts_mut(dst.as_mut_ptr(), 1) };
            unsafe { str::from_utf8_unchecked_mut(slice) }
        }
    }
}
impl AnyDualStress {
    pub const fn fmt_to(self, dst: &mut [u8; 9]) -> &mut str {
        let mut dst = UnsafeBuf::new(dst);

        // Format main into a 4-byte sub-buffer
        let main_len = self.main.fmt_to(dst.chunk()).len();
        dst.forward(main_len);

        if let Some(alt) = self.alt {
            // Append '/' as separator
            dst.push('/');

            // Format alt into a 4-byte sub-buffer
            let alt_len = alt.fmt_to(dst.chunk()).len();
            dst.forward(alt_len);
        }

        dst.finish()
    }
}

impl std::fmt::Display for AnyStress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; 4]).fmt(f)
    }
}
impl std::fmt::Display for AnyDualStress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; 9]).fmt(f)
    }
}

macro_rules! derive_fmt_impls {
    ($(
        $any:ty, $len:literal { $($t:ty),+ $(,)? }
    )+) => ($($(
        impl $t {
            pub const fn fmt_to(self, dst: &mut [u8; $len]) -> &mut str {
                <$any>::from(self).fmt_to(dst)
            }
        }
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                <$any>::from(*self).fmt(f)
            }
        }
    )+)+);
}
derive_fmt_impls! {
    AnyStress, 4 {
        NounStress, PronounStress, AdjectiveFullStress, AdjectiveShortStress, VerbPresentStress, VerbPastStress,
    }
    AnyDualStress, 9 {
        AdjectiveStress, VerbStress,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_any() {
        use AnyStress::*;

        assert_eq!(A.to_string(), "a");
        assert_eq!(B.to_string(), "b");
        assert_eq!(C.to_string(), "c");
        assert_eq!(D.to_string(), "d");
        assert_eq!(E.to_string(), "e");
        assert_eq!(F.to_string(), "f");
        assert_eq!(Ap.to_string(), "a′");
        assert_eq!(Bp.to_string(), "b′");
        assert_eq!(Cp.to_string(), "c′");
        assert_eq!(Dp.to_string(), "d′");
        assert_eq!(Ep.to_string(), "e′");
        assert_eq!(Fp.to_string(), "f′");
        assert_eq!(Cpp.to_string(), "c″");
        assert_eq!(Fpp.to_string(), "f″");
    }
    #[test]
    fn fmt_dual() {
        use AnyStress::*;

        assert_eq!(AnyDualStress::new(A, None).to_string(), "a");
        assert_eq!(AnyDualStress::new(F, None).to_string(), "f");
        assert_eq!(AnyDualStress::new(Bp, None).to_string(), "b′");
        assert_eq!(AnyDualStress::new(Ep, None).to_string(), "e′");
        assert_eq!(AnyDualStress::new(Cpp, None).to_string(), "c″");
        assert_eq!(AnyDualStress::new(Fpp, None).to_string(), "f″");
        assert_eq!(AnyDualStress::new(A, Some(A)).to_string(), "a/a");
        assert_eq!(AnyDualStress::new(A, Some(Fp)).to_string(), "a/f′");
        assert_eq!(AnyDualStress::new(Cp, Some(E)).to_string(), "c′/e");
        assert_eq!(AnyDualStress::new(Fpp, Some(Cpp)).to_string(), "f″/c″");
    }
}
