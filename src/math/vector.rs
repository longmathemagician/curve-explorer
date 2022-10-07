use std::ops::{Add, Mul, Sub};

use crate::math::common::format_f64;

use super::matrix::Matrix;

#[derive(Copy, Clone)]
pub struct Vector<const N: usize> {
    pub b: [f64; N],
}

impl<const N: usize> Vector<N> {
    pub const ZERO: Vector<N> = Self { b: [0.; N] };

    pub fn new(b: [f64; N]) -> Self {
        Self { b }
    }

    pub fn row(&self) -> Matrix<N, 1> {
        Matrix { e: [self.b] }
    }

    pub fn column(&self) -> Matrix<1, N> {
        let mut e = [[0.; 1]; N];
        for i in 0..N {
            e[i][0] = self.b[i];
        }
        Matrix { e }
    }

    /// Returns the Euclidean norm of the vector
    pub fn magnitude(&self) -> f64 {
        self.b.iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
    }

    /// Returns a normalized copy of the vector
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        let mut b = self.b.clone();

        for i in 0..N {
            b[i] *= mag.recip();
        }

        Self { b }
    }
}

impl Vector<2> {
    /// Returns the cross product of the two vectors
    pub fn cross(&self, rhs: &Vector<2>) -> f64 {
        self.b[0] * rhs.b[1] - self.b[1] * rhs.b[0]
    }
}

impl<const N: usize> core::fmt::Display for Vector<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        assert_ne!(N, 0);
        let mut output = String::from("(");
        for e in self.b {
            output.push_str(&format!("{}, ", format_f64(e, 7)));
        }
        output.pop();
        output.pop();
        output.push_str(")");
        f.write_str(&output)
    }
}

impl<const N: usize> Add<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn add(self, rhs: Vector<N>) -> Self::Output {
        let mut b = self.b.clone();
        for i in 0..N {
            b[i] += rhs.b[i];
        }

        Self::Output { b }
    }
}

impl<const N: usize> Sub<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn sub(self, rhs: Vector<N>) -> Self::Output {
        let mut b = self.b.clone();
        for i in 0..N {
            b[i] -= rhs.b[i];
        }

        Self::Output { b }
    }
}

impl<const N: usize> Mul<f64> for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut b = self.b.clone();
        for i in 0..N {
            b[i] *= rhs;
        }

        Self::Output { b }
    }
}

impl<const N: usize> Mul<Vector<N>> for f64 {
    type Output = Vector<N>;

    fn mul(self, rhs: Vector<N>) -> Self::Output {
        let mut b = rhs.b.clone();
        for i in 0..N {
            b[i] *= self;
        }

        Self::Output { b }
    }
}
