use crate::Generic;

macro_rules! impl_identity {
    ( $( $ty:ty ),+ $(,)? ) => {
        $(
            impl Generic for $ty {
                type Repr = $ty;
                fn into_repr(self: Self) -> Self::Repr {
                    self
                }
                fn from_repr(repr: Self::Repr) -> Self {
                    repr
                }
            }
        )+
    }
}

#[rustfmt::skip]
impl_identity!(
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128,
);
