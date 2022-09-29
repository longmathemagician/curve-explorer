struct Complex {
    re: f32,
    im: f32,
}
impl Complex {
    fn new() -> Complex {
        Complex { re: 0.0, im: 0.0 }
    }
    fn from(re: impl Into<f32>) -> Self {
        Self {
            re: re.into(),
            im: 0.0,
        }
    }
    fn mag(&self) -> f32 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
}

impl std::ops::Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl std::ops::Div for Complex {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            re: (self.re * other.re + self.im * other.im)
                / (other.re * other.re + other.im * other.im),
            im: (self.im * other.re - self.re * other.im)
                / (other.re * other.re + other.im * other.im),
        }
    }
}

impl std::ops::AddAssign for Complex {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl std::ops::SubAssign for Complex {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

impl std::ops::MulAssign for Complex {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl std::ops::DivAssign for Complex {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            re: (self.re * other.re + self.im * other.im)
                / (other.re * other.re + other.im * other.im),
            im: (self.im * other.re - self.re * other.im)
                / (other.re * other.re + other.im * other.im),
        }
    }
}
