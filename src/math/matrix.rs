use std::ops::Mul;

use crate::math::common::format_f64;

#[derive(Debug)]
pub struct Matrix<const M: usize, const N: usize> {
    pub e: [[f64; M]; N],
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub const ZERO: Matrix<M, N> = Self { e: [[0.; M]; N] };

    pub const IDENTITY: Matrix<M, N> = Matrix::identity();
    const fn identity() -> Self {
        if M != N {
            panic!()
        }
        let mut e = [[0.; M]; N];
        let mut i = 0;
        while i < M {
            e[i][i] = 1.;
            i += 1;
        }
        Self { e }
    }
    pub fn new(e: [[f64; M]; N]) -> Self {
        Self { e }
    }

    pub fn transpose(&self) -> Self {
        todo!()
    }
}

impl Matrix<2, 2> {
    pub fn determinant(&self) -> f64 {
        self.e[0][0] * self.e[1][1] - self.e[0][1] * self.e[1][0]
    }
}

impl<const M: usize, const N: usize> core::fmt::Display for Matrix<M, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        assert_ne!(M, 0);
        assert_ne!(N, 0);
        let mut output = String::from("\n");
        for i in 0..N {
            output.push_str("|");
            for e in self.e[i] {
                output.push_str(&format!("{}, ", format_f64(e, 7)));
            }
            output.pop();
            output.pop();
            output.push_str("|\n");
        }
        f.write_str(&output)
    }
}

impl<const M: usize, const N: usize, const O: usize> Mul<Matrix<O, M>> for Matrix<M, N> {
    type Output = Matrix<O, N>;

    fn mul(self, x: Matrix<O, M>) -> Self::Output {
        let mut e = [[0.; O]; N];
        for i in 0..O {
            for j in 0..N {
                for k in 0..M {
                    e[j][i] += self.e[j][k] * x.e[k][i];
                }
            }
        }
        Self::Output { e }
    }
}
