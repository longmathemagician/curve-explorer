use crate::math::real::Real;
use std::fmt;
use std::ops::*;

use serde::{Deserialize, Serialize};

pub const _ZERO: Vec2<f32> = Vec2::new(0., 0.);

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Real> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub const ZERO: Vec2<f32> = Vec2::new(0., 0.);

    pub fn normalize(&self) -> Self {
        let magnitude = self.mag();

        *self / magnitude
    }

    pub fn normal(&self) -> Self {
        Vec2::new(-self.y, self.x)
    }

    // Return the Euclidean norm of the vector
    // TODO: Runtime type checking?
    pub fn mag(&self) -> T {
        let elem: (f32, f32) = (self.x.try_into().unwrap(), self.y.try_into().unwrap());
        T::from((elem.0.powi(2) + elem.1.powi(2)).sqrt())
    }
}

impl<T: Real> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Implement standard ops for Vec2<T> op Vec2<T>,
// Vec2<T> op T, and (some :( ) T op Vec2<T>.

// Negation: -Vec2<T> -> Vec2<T>
impl<T> Neg for Vec2<T>
where
    T: Real,
{
    type Output = Vec2<T>;
    fn neg(self) -> Vec2<T> {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

// Addition: Vec2<T> + Vec2<T> -> Vec2<T>
impl<T> Add<Vec2<T>> for Vec2<T>
where
    T: Real,
{
    type Output = Vec2<T>;
    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Addition: Vec2<T> + T -> Vec2<T>
impl<T> Add<T> for Vec2<T>
where
    T: Real,
{
    type Output = Vec2<T>;
    fn add(self, rhs: T) -> Vec2<T> {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

// Addition: T + Vec2<T> -> Vec2<T>
impl<T> Add<Vec2<T>> for f32
where
    T: Real,
{
    type Output = Vec2<T>;
    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: rhs.x + T::from(self),
            y: rhs.y + T::from(self),
        }
    }
}

// Subtraction: Vec2<T> - Vec2<T> -> Vec2<T>
impl<T> Sub<Vec2<T>> for Vec2<T>
where
    T: Real,
{
    type Output = Vec2<T>;
    fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// Subtraction: Vec2<T> - T -> Vec2<T>
impl<T> Sub<T> for Vec2<T>
where
    T: Real,
{
    type Output = Vec2<T>;
    fn sub(self, rhs: T) -> Vec2<T> {
        Vec2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

// Subtraction: T - Vec2<T> -> Vec2<T>
impl<T> Sub<Vec2<T>> for f32
where
    T: Real,
{
    type Output = Vec2<T>;
    fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: -rhs.x + T::from(self),
            y: -rhs.y + T::from(self),
        }
    }
}

// ~~Multiplication~~ inner product: Vec2<T> * Vec2<T> -> T
impl<T> Mul<Vec2<T>> for Vec2<T>
where
    T: Real,
{
    type Output = T;
    fn mul(self, rhs: Vec2<T>) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}

// Scalar multiplication: Vec2<T> * T -> Vec2<T> where T: Real
impl<T> Mul<T> for Vec2<T>
where
    T: Real,
{
    type Output = Vec2<T>;
    fn mul(self, rhs: T) -> Vec2<T> {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// Scalar multiplication: Vec2<f32> * rhs: usize -> Vec2<f32>
impl Mul<usize> for Vec2<f32> {
    type Output = Vec2<f32>;
    fn mul(self, rhs: usize) -> Vec2<f32> {
        let scalar = rhs as f32;
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// Scalar multiplication: T * Vec2<T> -> Vec2<T>
impl<T> Mul<Vec2<T>> for f32
where
    T: Real,
{
    type Output = Vec2<T>;
    fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: rhs.x * T::from(self),
            y: rhs.y * T::from(self),
        }
    }
}

// Scalar multiplication: U * Vec2<T> -> Vec2<T> where T: Real, U: usize
impl<T> Mul<Vec2<T>> for usize
where
    T: Real,
{
    type Output = Vec2<T>;
    fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
        let scalar = T::from(self as f32);
        Vec2 {
            x: rhs.x * scalar,
            y: rhs.y * scalar,
        }
    }
}

// ~~Division~~ cross product in R2: Vec2<T> / Vec2<T> -> T
impl<T> Div<Vec2<T>> for Vec2<T>
where
    T: Real,
{
    type Output = T;
    fn div(self, rhs: Vec2<T>) -> T {
        self.x * rhs.y - self.y * rhs.x
    }
}

// Scalar division: Vec2<T> / T -> Vec2<T>
impl<T> Div<T> for Vec2<T>
where
    T: Real,
{
    type Output = Vec2<T>;
    fn div(self, rhs: T) -> Vec2<T> {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

// Scalar division: T / Vec2<T> -> Vec2<T>
impl<T> Div<Vec2<T>> for f32
where
    T: Real,
{
    type Output = Vec2<T>;
    fn div(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: T::from(self) / rhs.x,
            y: T::from(self) / rhs.y,
        }
    }
}

#[cfg(test)]
mod vec2_tests {
    use crate::math::vec2::Vec2;

    #[test]
    fn ops() {
        // TODO: More tests
        const ZERO: Vec2<f32> = Vec2::new(0., 0.);
        let a = Vec2::new(7., 13.);
        let b = Vec2::new(5., 17.);

        // Check vector addition
        let a_plus_b = Vec2::new(12., 30.);
        assert_eq!(a + ZERO, a);
        assert_eq!(a + b, a_plus_b);

        // Check vector subtraction
        let a_minus_b = Vec2::new(2., -4.);
        assert_eq!(a - ZERO, a);
        assert_eq!(a - b, a_minus_b);
    }
}
