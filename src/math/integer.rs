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

trait_alias!(Integer =
    'static +
    Display +
    Copy +
    Sized +
    PartialOrd +
    Into<usize> +
    From<usize> +
    Mul<Output = Self> + MulAssign +
);
