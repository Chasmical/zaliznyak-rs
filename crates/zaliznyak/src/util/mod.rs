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
        impl const TryFrom<$from> for $to {
            type Error = $err;
            fn try_from(value: $from) -> Result<Self, Self::Error> {
                Ok(match value {
                    $( <$from>::$variant => <$to>::$variant, )+
                    _ => return Err(Self::Error {}),
                })
            }
        }
        impl const From<$to> for $from {
            fn from(value: $to) -> Self {
                match value { $( <$to>::$variant => <$from>::$variant, )+ }
            }
        }
    );
}

pub(crate) use enum_conversion;
