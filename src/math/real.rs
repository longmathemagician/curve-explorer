use std::fmt::Display;
use std::ops::*;

macro_rules! items {
    ($($item:item)*) => ($($item)*);
}

macro_rules! trait_alias {
    ($name:ident = $($base:tt)+) => {
        items! {
            pub trait $name: $($base)+ { }
            impl<T: $($base)+> $name for T { }
        }
    };
}

trait_alias!(Real =
    'static +
    Display +
    Copy +
    Sized +
    PartialOrd +
    Into<f32> +
    From<f32> + // TODO: Get away from these?
    Neg<Output = Self> +
    Add<Output = Self> + AddAssign +
    Sub<Output = Self> + SubAssign +
    Mul<Output = Self> + MulAssign +
    Div<Output = Self> + DivAssign +
);
