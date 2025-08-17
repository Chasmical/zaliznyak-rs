mod unsafe_buf;
pub(crate) use unsafe_buf::*;
mod const_try;
pub(crate) use const_try::*;

macro_rules! enum_conversion {
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

pub(crate) use enum_conversion;
