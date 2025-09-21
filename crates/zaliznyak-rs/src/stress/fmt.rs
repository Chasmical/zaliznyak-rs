use crate::{
    stress::{
        AdjectiveFullStress, AdjectiveShortStress, AdjectiveStress, AnyDualStress, AnyStress,
        NounStress, PronounStress, VerbPastStress, VerbPresentStress, VerbStress,
    },
    util::UnsafeBuf,
};

/// The maximum byte length of a formatted [`AnyDualStress`].
///
/// Longest form: f″/f″ (9 bytes, 5 chars)
pub const DUAL_STRESS_MAX_LEN: usize = 9;

impl AnyStress {
    /// Formats this stress schema as UTF-8 into the provided byte buffer, and then returns
    /// the subslice of the buffer that contains the encoded string.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::AnyStress;
    ///
    /// let mut buf = String::new();
    /// buf.push_str(AnyStress::Bp.fmt_to(&mut [0; 4]));
    /// assert_eq!(buf, "b′");
    /// ```
    #[must_use]
    pub const fn fmt_to(self, dst: &mut [u8; 4]) -> &mut str {
        // Write the latin letter
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
    /// Formats this dual stress schema as UTF-8 into the provided byte buffer, and then returns
    /// the subslice of the buffer that contains the encoded string.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::{AnyDualStress, AnyStress};
    ///
    /// let s = AnyDualStress::new(AnyStress::Fpp, Some(AnyStress::B));
    ///
    /// let mut buf = String::new();
    /// buf.push_str(s.fmt_to(&mut [0; 9]));
    /// assert_eq!(buf, "f″/b");
    /// ```
    #[must_use]
    pub const fn fmt_to(self, dst: &mut [u8; 9]) -> &mut str {
        let mut dst = UnsafeBuf::new(dst);

        // Format main into buffer
        dst.push_fmt2(self.main, AnyStress::fmt_to);

        if let Some(alt) = self.alt {
            // Append '/' as separator
            dst.push('/');
            // Format alt into buffer
            dst.push_fmt2(alt, AnyStress::fmt_to);
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

macro_rules! derive_simple_fmt_impls {
    ($($t:ty),+ $(,)?) => ($(
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                AnyStress::from(*self).fmt(f)
            }
        }
    )+);
}
derive_simple_fmt_impls! {
    NounStress, PronounStress, AdjectiveFullStress, AdjectiveShortStress, VerbPresentStress, VerbPastStress,
}

impl std::fmt::Display for AdjectiveStress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        AnyDualStress::from(*self).abbr_adj().fmt(f)
    }
}
impl std::fmt::Display for VerbStress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        AnyDualStress::from(*self).abbr_verb().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_any() {
        assert_eq!(AnyStress::A.to_string(), "a");
        assert_eq!(AnyStress::B.to_string(), "b");
        assert_eq!(AnyStress::C.to_string(), "c");
        assert_eq!(AnyStress::D.to_string(), "d");
        assert_eq!(AnyStress::E.to_string(), "e");
        assert_eq!(AnyStress::F.to_string(), "f");
        assert_eq!(AnyStress::Ap.to_string(), "a′");
        assert_eq!(AnyStress::Bp.to_string(), "b′");
        assert_eq!(AnyStress::Cp.to_string(), "c′");
        assert_eq!(AnyStress::Dp.to_string(), "d′");
        assert_eq!(AnyStress::Ep.to_string(), "e′");
        assert_eq!(AnyStress::Fp.to_string(), "f′");
        assert_eq!(AnyStress::Cpp.to_string(), "c″");
        assert_eq!(AnyStress::Fpp.to_string(), "f″");
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
    #[test]
    fn fmt_adj() {
        assert_eq!(AdjectiveStress::A_A.to_string(), "a");
        assert_eq!(AdjectiveStress::B_B.to_string(), "b");
        assert_eq!(AdjectiveStress::A_Ap.to_string(), "a′");
        assert_eq!(AdjectiveStress::B_Bp.to_string(), "b′");
        assert_eq!(AdjectiveStress::B_A.to_string(), "b/a");
        assert_eq!(AdjectiveStress::A_Cp.to_string(), "a/c′");
        assert_eq!(AdjectiveStress::B_Cpp.to_string(), "b/c″");
    }
    #[test]
    fn fmt_verb() {
        assert_eq!(VerbStress::A_A.to_string(), "a");
        assert_eq!(VerbStress::B_A.to_string(), "b");
        assert_eq!(VerbStress::C_A.to_string(), "c");
        assert_eq!(VerbStress::A_C.to_string(), "a/c");
        assert_eq!(VerbStress::B_B.to_string(), "b/b");
        assert_eq!(VerbStress::C_Cpp.to_string(), "c/c″");
        assert_eq!(VerbStress::Cp_C.to_string(), "c′/c");
    }
}
