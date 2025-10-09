use crate::word::{Utf8Letter, WordBuf};
use thiserror::Error;

/// Error type for parsing [`WordBuf`] from a string.
#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum ParseWordError {
    /// The string contained non-lowercase-cyrillic characters.
    #[error("string contains non-lowercase-cyrillic characters")]
    NonCyrillic,
    /// The string does not specify stress, and it can't be inferred automatically.
    #[error("string does not specify stress")]
    NoStress,
}

fn is_cyrillic(s: &str) -> bool {
    if let (chunks, []) = s.as_bytes().as_chunks::<2>()
        && chunks.iter().all(|ch| Utf8Letter::from_utf8(*ch).is_some())
    {
        true
    } else {
        false
    }
}

pub(super) fn find_implicit_insert_stress_pos(word: &[Utf8Letter]) -> Option<usize> {
    let mut iter = word.iter().copied().enumerate().filter(|x| x.1.is_vowel());

    let (first_idx, first_vowel) = iter.next()?;
    let mut result = Some(first_idx + 1);

    if first_vowel != Utf8Letter::Ё {
        for (next_idx, next_vowel) in iter {
            if next_vowel == Utf8Letter::Ё {
                return Some(next_idx + 1);
            }
            result = None;
        }
    }
    result
}

// TODO: constify WordBuf::from_str?
impl std::str::FromStr for WordBuf {
    type Err = ParseWordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = String::from(s);

        let mut stress_pos = s.find(['\u{0300}', '\u{0301}', '\'']);
        if let Some(accent_pos) = &mut stress_pos {
            s.remove(*accent_pos);
            *accent_pos /= 2;
        }

        let mut stem_len = s.find('-');
        if let Some(dash_pos) = &mut stem_len {
            s.remove(*dash_pos);
            *dash_pos /= 2;
        }

        if !is_cyrillic(&s) {
            return Err(ParseWordError::NonCyrillic);
        }

        let char_len = s.len() / 2;
        let mut word = Self::with_capacity(char_len);

        // SAFETY: The allocated buffer is guaranteed to have enough capacity to copy into.
        unsafe {
            let dst = std::slice::from_raw_parts_mut(word.buf.as_mut_ptr().cast(), s.len());
            dst.copy_from_slice(s.as_bytes());
            word.buf.set_len(char_len);
        }
        word.stem_len = stem_len.unwrap_or(char_len);

        word.stress_at = stress_pos
            .or_else(|| find_implicit_insert_stress_pos(word.as_letters()))
            .ok_or(ParseWordError::NoStress)?;

        Ok(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::word::Utf8Letter::*;

    #[test]
    #[rustfmt::skip]
    fn from_str() {
        // Explicit stress and explicit ending
        assert_eq!(
            "я́блок-о".parse(),
            Ok(WordBuf {
                buf: [Я, Б, Л, О, К, О].into(),
                stem_len: 5, stress_at: 1,
            }),
        );
        assert_eq!(
            "гру̀ш-а".parse(),
            Ok(WordBuf {
                buf: [Г, Р, У, Ш, А].into(),
                stem_len: 4, stress_at: 3,
            }),
        );
        assert_eq!(
            "шестерн-я'".parse(),
            Ok(WordBuf {
                buf: [Ш, Е, С, Т, Е, Р, Н, Я].into(),
                stem_len: 7, stress_at: 8,
            }),
        );

        // Implicit stress on the only vowel
        assert_eq!(
            "род".parse(),
            Ok(WordBuf {
                buf: [Р, О, Д].into(),
                stem_len: 3, stress_at: 2,
            }),
        );
        assert_eq!(
            "рж-и".parse(),
            Ok(WordBuf {
                buf: [Р, Ж, И].into(),
                stem_len: 2, stress_at: 3,
            }),
        );

        // Implicit and explicit stress, with 'ё' present
        assert_eq!(
            "сестёр".parse(),
            Ok(WordBuf {
                buf: [С, Е, С, Т, Ё, Р].into(),
                stem_len: 6, stress_at: 5,
            }),
        );
        assert_eq!(
            "сёр-а́".parse(),
            Ok(WordBuf {
                buf: [С, Ё, Р, А].into(),
                stem_len: 3, stress_at: 4,
            }),
        );
    }
}
