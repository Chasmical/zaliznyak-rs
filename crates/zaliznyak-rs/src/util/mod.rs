mod const_utils;
mod unsafe_buf;
mod unsafe_parser;

pub(crate) use const_utils::*;
pub(crate) use unsafe_buf::*;
pub(crate) use unsafe_parser::*;

macro_rules! enum_conversion {
    (
        $from:ty => $to:ty {
            $($variant:ident),+ $(,)?
        }
    ) => (
        impl const From<$from> for $to {
            fn from(value: $from) -> Self {
                match value { $( <$from>::$variant => <$to>::$variant, )+ }
            }
        }
        impl const From<$to> for $from {
            fn from(value: $to) -> Self {
                match value { $( <$to>::$variant => <$from>::$variant, )+ }
            }
        }
    );
    (
        $from:ty => $to:ty {
            $($variant:ident),+ $(,)?
        } else {
            $err:ty
        }
    ) => (
        impl const From<$from> for $to {
            fn from(value: $from) -> Self {
                match value { $( <$from>::$variant => <$to>::$variant, )+ }
            }
        }
        impl const TryFrom<$to> for $from {
            type Error = $err;
            fn try_from(value: $to) -> Result<Self, Self::Error> {
                Ok(match value {
                    $( <$to>::$variant => <$from>::$variant, )+
                    _ => return Err(Self::Error {}),
                })
            }
        }
    );
}

pub(crate) const trait _ToUtf8 {
    fn _len_utf8(&self) -> usize;
    fn _encode_utf8(&self, dst: &mut [u8]);
}
impl const _ToUtf8 for char {
    fn _len_utf8(&self) -> usize {
        self.len_utf8()
    }
    fn _encode_utf8(&self, dst: &mut [u8]) {
        self.encode_utf8(dst);
    }
}
impl const _ToUtf8 for &str {
    fn _len_utf8(&self) -> usize {
        self.len()
    }
    fn _encode_utf8(&self, dst: &mut [u8]) {
        dst.copy_from_slice(self.as_bytes());
    }
}

macro_rules! utf8_bytes {
    ($ch:literal) => {{
        let mut buf = [0; $crate::util::_ToUtf8::_len_utf8(&$ch)];
        $crate::util::_ToUtf8::_encode_utf8(&$ch, &mut buf);
        buf
    }};
}

pub(crate) use {enum_conversion, utf8_bytes};
