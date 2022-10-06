use crate::math::real::Real;
use crate::Vec2;
use std::ops::Mul;

#[derive(Debug, Copy, Clone)]
pub struct Matrix2x2<T> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub d: T,
}

impl<T: Real> Matrix2x2<T> {
    pub fn identity() -> Self {
        Self {
            a: T::from(1.0),
            b: T::from(0.0),
            c: T::from(0.0),
            d: T::from(1.0),
        }
    }
    pub fn rotation(&self, theta: f32) -> Self {
        let rotation_matrix = Matrix2x2 {
            a: T::from(theta.cos()),
            b: T::from(-theta.sin()),
            c: T::from(theta.sin()),
            d: T::from(theta.cos()),
        };

        *self * rotation_matrix
    }

    pub fn scale(&self, scale_factor: f32) -> Self {
        let scale_matrix = Matrix2x2 {
            a: T::from(scale_factor),
            b: T::from(0.0),
            c: T::from(0.0),
            d: T::from(scale_factor),
        };

        scale_matrix * *self
    }

    pub fn determinant(&self) -> T {
        T::from(1.0) / (self.a * self.d - self.b * self.c)
    }

    pub fn inverse(&self) -> Self {
        let det = self.determinant();
        Self {
            a: det * self.d,
            b: det * (T::from(0.0) - self.b),
            c: det * (T::from(0.0) - self.c),
            d: det * (self.a),
        }
    }
}

impl<T> Mul<Vec2<T>> for Matrix2x2<T>
where
    T: Real,
{
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::new(
            self.a * rhs.x + self.b * rhs.y,
            self.c * rhs.x + self.d * rhs.y,
        )
    }
}

impl<T> Mul<Matrix2x2<T>> for Matrix2x2<T>
where
    T: Real,
{
    type Output = Matrix2x2<T>;

    fn mul(self, rhs: Matrix2x2<T>) -> Self::Output {
        Matrix2x2 {
            a: self.a * rhs.a + self.b * rhs.c,
            b: self.a * rhs.b + self.b * rhs.d,
            c: self.c * rhs.a + self.d * rhs.c,
            d: self.c * rhs.b + self.d * rhs.d,
        }
    }
}
