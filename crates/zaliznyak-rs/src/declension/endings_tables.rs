#![allow(non_upper_case_globals)]
use crate::{util::slice_find, word::Utf8Letter};

mod endings {
    use crate::word::Utf8Letter::{self, *};

    // All endings of nouns, pronouns and adjectives in one 54-char slice
    #[rustfmt::skip]
    pub const ENDINGS: [Utf8Letter; 54] = [
        А,Я,М,И,М,И,Е,Е,Г,О,В,Ы,М,И,Х,Е,М,У,Ю,Ь,Ю,Ю,Ы,Е,В,Я,Я,Х,А,М,И,Й,О,Г,О,Й,О,Е,Й,О,М,У,Ы,Х,Ы,Й,Ё,В,А,Х,Ё,Й,Ё,М,
    ];
}

// [case:6] [number:2] [gender:3] [stem type:8] = [total:288]
#[rustfmt::skip]
pub(crate) const NOUN_LOOKUP: [Endings; 288] = [
    //    stem types: 1,   2,   3,   4,   5,   6,   7,   8
    /* nom sg masc */ NIL, ь,   NIL, NIL, NIL, й,   й,   ь,
    /* nom sg n    */ о,   е/ё, о,   е/о, е/о, е/ё, е/ё, о,
    /* nom sg fem  */ а,   я,   а,   а,   а,   я,   я,   ь,
    //    stem types: 1, 2, 3, 4, 5, 6, 7, 8
    /* nom pl masc */ ы, и, и, и, ы, и, и, и,
    /* nom pl n    */ а, я, а, а, а, я, я, а,
    /* nom pl fem  */ ы, и, и, и, ы, и, и, и,

    //    stem types: 1, 2, 3, 4, 5, 6, 7, 8
    /* gen sg masc */ а, я, а, а, а, я, я, и,
    /* gen sg n    */ а, я, а, а, а, я, я, а,
    /* gen sg fem  */ ы, и, и, и, ы, и, и, и,
    //    stem types: 1,   2,    3,   4,      5,     6,     7,     8
    /* gen pl masc */ ов,  ей,   ов,  ей,     ев/ов, ев/ёв, ев/ёв, ей,
    /* gen pl n    */ NIL, ь/ей, NIL, NIL/ей, NIL,   й,     й,     NIL,
    /* gen pl fem  */ NIL, ь/ей, NIL, NIL/ей, NIL,   й,     й,     ей,

    //    stem types: 1, 2, 3, 4, 5, 6, 7,   8
    /* dat sg masc */ у, ю, у, у, у, ю, ю,   и,
    /* dat sg n    */ у, ю, у, у, у, ю, ю,   у,
    /* dat sg fem  */ е, е, е, е, е, е, и/е, и,
    //    stem types: 1,  2,  3,  4,  5,  6,  7,  8
    /* dat pl masc */ ам, ям, ам, ам, ам, ям, ям, ям,
    /* dat pl n    */ ам, ям, ам, ам, ам, ям, ям, ам,
    /* dat pl fem  */ ам, ям, ам, ам, ам, ям, ям, ям,

    //    stem types: 1,   2,   3,   4,   5,   6,   7,   8
    /* acc sg masc */ ACC, ACC, ACC, ACC, ACC, ACC, ACC, ACC,
    /* acc sg n    */ ACC, ACC, ACC, ACC, ACC, ACC, ACC, ACC,
    /* acc sg fem  */ у,   ю,   у,   у,   у,   ю,   ю,   ь,
    //    stem types: 1,   2,   3,   4,   5,   6,   7,   8
    /* acc pl masc */ ACC, ACC, ACC, ACC, ACC, ACC, ACC, ACC,
    /* acc pl n    */ ACC, ACC, ACC, ACC, ACC, ACC, ACC, ACC,
    /* acc pl fem  */ ACC, ACC, ACC, ACC, ACC, ACC, ACC, ACC,

    //    stem types: 1,  2,     3,  4,     5,     6,     7,     8
    /* ins sg masc */ ом, ем/ём, ом, ем/ом, ем/ом, ем/ём, ем/ём, ем/ём,
    /* ins sg n    */ ом, ем/ём, ом, ем/ом, ем/ом, ем/ём, ем/ём, ом,
    /* ins sg fem  */ ой, ей/ёй, ой, ей/ой, ей/ой, ей/ёй, ей/ёй, ью,
    //    stem types: 1,   2,   3,   4,   5,   6,   7,   8
    /* ins pl masc */ ами, ями, ами, ами, ами, ями, ями, ями,
    /* ins pl n    */ ами, ями, ами, ами, ами, ями, ями, ами,
    /* ins pl fem  */ ами, ями, ами, ами, ами, ями, ями, ями,

    //    stem types: 1, 2, 3, 4, 5, 6, 7,   8
    /* prp sg masc */ е, е, е, е, е, е, и/е, и,
    /* prp sg n    */ е, е, е, е, е, е, и/е, и,
    /* prp sg fem  */ е, е, е, е, е, е, и/е, и,
    //    stem types: 1,  2,  3,  4,  5,  6,  7,  8
    /* prp pl masc */ ах, ях, ах, ах, ах, ях, ях, ях,
    /* prp pl n    */ ах, ях, ах, ах, ах, ях, ях, ах,
    /* prp pl fem  */ ах, ях, ах, ах, ах, ях, ях, ях,
];

// [case:6] [gender|plural:4] [stem type:4] = [total:96]
#[rustfmt::skip]
pub(crate) const PRONOUN_LOOKUP: [Endings; 96] = [
    // stem types: 1,   2,   4,   6,
    /* nom masc */ NIL, ь,   NIL, й,
    /* nom n    */ о,   е/ё, е/о, е/ё,
    /* nom fem  */ а,   я,   а,   я,
    /* nom pl   */ ы,   и,   и,   и,

    // stem types: 1,  2,  4,       6,
    /* gen masc */ а,  я,  его/ого, его,
    /* gen n    */ а,  я,  его/ого, его,
    /* gen fem  */ ой, ей, ей/ой,   ей,
    /* gen pl   */ ых, их, их,      их,

    // stem types: 1,  2,  4,       6,
    /* dat masc */ у,  ю,  ему/ому, ему,
    /* dat n    */ у,  ю,  ему/ому, ему,
    /* dat fem  */ ой, ей, ей/ой,   ей,
    /* dat pl   */ ым, им, им,      им,

    // stem types: 1,   2,   4,   6,
    /* acc masc */ ACC, ACC, ACC, ACC,
    /* acc n    */ ACC, ACC, ACC, ACC,
    /* acc fem  */ у,   ю,   у,   ю,
    /* acc pl   */ ACC, ACC, ACC, ACC,

    // stem types: 1,   2,   4,     6,
    /* ins masc */ ым,  им,  им,    им,
    /* ins n    */ ым,  им,  им,    им,
    /* ins fem  */ ой,  ей,  ей/ой, ей,
    /* ins pl   */ ыми, ими, ими,   ими,

    // stem types: 1,  2,     4,     6,
    /* prp masc */ ом, ем/ём, ем/ом, ем/ём,
    /* prp n    */ ом, ем/ём, ем/ом, ем/ём,
    /* prp fem  */ ой, ей,    ей/ой, ей,
    /* prp pl   */ ых, их,    их,    их,
];

// [case+short form:7] [gender|plural:4] [stem type:6] = [total:168]
#[rustfmt::skip]
pub(crate) const ADJECTIVE_LOOKUP: [Endings; 168] = [
    // stem types: 1,     2,  3,     4,     5,     6
    /* nom masc */ ый/ой, ий, ий/ой, ий/ой, ый/ой, ий,
    /* nom n    */ ое,    ее, ое,    ее/ое, ее/ое, ее,
    /* nom fem  */ ая,    яя, ая,    ая,    ая,    яя,
    /* nom pl   */ ые,    ие, ие,    ие,    ые,    ие,

    // stem types: 1,   2,   3,   4,       5,       6
    /* gen masc */ ого, его, ого, его/ого, его/ого, его,
    /* gen n    */ ого, его, ого, его/ого, его/ого, его,
    /* gen fem  */ ой,  ей,  ой,  ей/ой,   ей/ой,   ей,
    /* gen pl   */ ых,  их,  их,  их,      ых,      их,

    // stem types: 1,   2,   3,   4,       5,       6
    /* dat masc */ ому, ему, ому, ему/ому, ему/ому, ему,
    /* dat n    */ ому, ему, ому, ему/ому, ему/ому, ему,
    /* dat fem  */ ой,  ей,  ой,  ей/ой,   ей/ой,   ей,
    /* dat pl   */ ым,  им,  им,  им,      ым,      им,

    // stem types: 1,   2,   3,   4,   5,   6
    /* acc masc */ ACC, ACC, ACC, ACC, ACC, ACC,
    /* acc n    */ ACC, ACC, ACC, ACC, ACC, ACC,
    /* acc fem  */ ую,  юю,  ую,  ую,  ую,  юю,
    /* acc pl   */ ACC, ACC, ACC, ACC, ACC, ACC,

    // stem types: 1,   2,   3,   4,     5,     6
    /* ins masc */ ым,  им,  им,  им,    ым,    им,
    /* ins n    */ ым,  им,  им,  им,    ым,    им,
    /* ins fem  */ ой,  ей,  ой,  ей/ой, ей/ой, ей,
    /* ins pl   */ ыми, ими, ими, ими,   ыми,   ими,

    // stem types: 1,  2,  3,  4,     5,     6
    /* prp masc */ ом, ем, ом, ем/ом, ем/ом, ем,
    /* prp n    */ ом, ем, ом, ем/ом, ем/ом, ем,
    /* prp fem  */ ой, ей, ой, ей/ой, ей/ой, ей,
    /* prp pl   */ ых, их, их, их,    ых,    их,

    // stem types: 1,   2,   3,   4,   5,   6
    /* srt masc */ NIL, ь,   NIL, NIL, NIL, й,
    /* srt n    */ о,   е/ё, о,   е/о, е/о, е/ё,
    /* srt fem  */ а,   я,   а,   а,   а,   я,
    /* srt pl   */ ы,   и,   и,   и,   ы,   и,
];

// Define all the possible ending constants
macro_rules! define_endings {
    ($($ident:ident)*) => ($(
        const $ident: Endings = Endings::encode(stringify!($ident));
    )*);
}
define_endings! {
    о е ов ы ей й ё ём ой ёй а ам ами и я ям ями ем у ю ах ях ом ев ёв ь ью // nouns
    ое его ого ые ее ий ая ие ему ую юю яя ый ых ым ыми их ому им ими // pronouns, adjectives
}

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub(super) struct Endings(u8, u8);

// Special constant for accusative case endings that depend on animacy.
const ACC: Endings = Endings(0x00, 0x00);
// Special constant for "" (null) ending (pos=1, len=0).
const NIL: Endings = Endings(0x01, 0x01);

impl Endings {
    pub const fn is_acc(&self) -> bool {
        self.0 == 0
    }
    pub const fn invariant(&self) -> bool {
        self.0 == self.1
    }

    const fn encode(s: &str) -> Self {
        // Encoding format:
        //   11_xxxxxx - length   (in increments of 2 bytes; UTF-16)
        //   xx_111111 - position (in increments of 2 bytes; UTF-16)

        let mut letters = [Utf8Letter::А; 3];
        unsafe { std::ptr::copy_nonoverlapping(s.as_ptr(), letters.as_mut_ptr().cast(), s.len()) };
        let letters = &letters[..s.len() / 2];

        let start = slice_find(&endings::ENDINGS, letters).unwrap();
        let encoded = ((letters.len() << 6) | start) as u8;
        Self(encoded, encoded)
    }

    // FIXME(const-hack): use const closure here instead for determining stress
    pub const fn get(self, is_stressed: bool) -> &'static [Utf8Letter] {
        // Ensure that the accusative case is handled properly by outside code
        debug_assert!(!self.is_acc());

        let index = if is_stressed { self.1 } else { self.0 };
        let start = (index & 0x3F) as usize;
        let len = (index >> 6) as usize;
        unsafe { endings::ENDINGS.get_unchecked(start..start + len) }
    }
}

impl const std::ops::Div for Endings {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0, rhs.1)
    }
}
