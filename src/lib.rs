use std::{cmp::Ordering, fmt::Display, ops::*};

#[derive(Clone, Copy, Debug, Default)]
pub struct StupidFloat64(f64);

impl Display for StupidFloat64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_nan() {
            let sign = if self.0.to_bits() >> 63 == 1 {
                '-'
            } else {
                '+'
            };
            write!(f, "{sign}NaN")
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl PartialEq for StupidFloat64 {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_nan() && other.0.is_nan() {
            let my_bits = self.0.to_bits();
            let your_bits = other.0.to_bits();
            my_bits == your_bits
        } else {
            self.0 == other.0
        }
    }
}
impl Eq for StupidFloat64 {}

impl PartialOrd for StupidFloat64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0.is_nan() && other.0.is_nan() {
            let my_bits = self.0.to_bits();
            let your_bits = other.0.to_bits();
            let my_sign_bit = my_bits >> 63;
            let your_sign_bit = your_bits >> 63;

            let my_mantissa = my_bits | (1 << 63);
            let your_mantissa = your_bits | (1 << 63);

            Some(match (my_sign_bit, your_sign_bit) {
                (0, 0) => my_mantissa.cmp(&your_mantissa),
                (0, 1) => Ordering::Greater,
                (1, 0) => Ordering::Less,
                (1, 1) => your_mantissa.cmp(&my_mantissa),
                _ => unreachable!(),
            })
        } else {
            self.0.partial_cmp(&other.0)
        }
    }
}

impl Add for StupidFloat64 {
    type Output = StupidFloat64;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub for StupidFloat64 {
    type Output = StupidFloat64;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl Mul for StupidFloat64 {
    type Output = StupidFloat64;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
impl Div for StupidFloat64 {
    type Output = StupidFloat64;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}
impl Rem for StupidFloat64 {
    type Output = StupidFloat64;
    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}
impl Neg for StupidFloat64 {
    type Output = StupidFloat64;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl AddAssign for StupidFloat64 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}
impl SubAssign for StupidFloat64 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0)
    }
}
impl MulAssign for StupidFloat64 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self(self.0 * rhs.0)
    }
}
impl DivAssign for StupidFloat64 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self(self.0 / rhs.0)
    }
}
impl RemAssign for StupidFloat64 {
    fn rem_assign(&mut self, rhs: Self) {
        *self = Self(self.0 % rhs.0)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StupidFloat32(f32);

impl Display for StupidFloat32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_nan() {
            let sign = if self.0.to_bits() >> 31 == 1 {
                '-'
            } else {
                '+'
            };
            write!(f, "{sign}NaN")
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl PartialEq for StupidFloat32 {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_nan() && other.0.is_nan() {
            let my_bits = self.0.to_bits();
            let your_bits = other.0.to_bits();
            my_bits == your_bits
        } else {
            self.0 == other.0
        }
    }
}
impl Eq for StupidFloat32 {}

impl PartialOrd for StupidFloat32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0.is_nan() && other.0.is_nan() {
            let my_bits = self.0.to_bits();
            let your_bits = other.0.to_bits();
            let my_sign_bit = my_bits >> 31;
            let your_sign_bit = your_bits >> 31;

            let my_mantissa = my_bits | (1 << 31);
            let your_mantissa = your_bits | (1 << 31);

            Some(match (my_sign_bit, your_sign_bit) {
                (0, 0) => my_mantissa.cmp(&your_mantissa),
                (0, 1) => Ordering::Greater,
                (1, 0) => Ordering::Less,
                (1, 1) => your_mantissa.cmp(&my_mantissa),
                _ => unreachable!(),
            })
        } else {
            self.0.partial_cmp(&other.0)
        }
    }
}

impl Add for StupidFloat32 {
    type Output = StupidFloat32;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub for StupidFloat32 {
    type Output = StupidFloat32;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl Mul for StupidFloat32 {
    type Output = StupidFloat32;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
impl Div for StupidFloat32 {
    type Output = StupidFloat32;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}
impl Rem for StupidFloat32 {
    type Output = StupidFloat32;
    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}
impl Neg for StupidFloat32 {
    type Output = StupidFloat32;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl AddAssign for StupidFloat32 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}
impl SubAssign for StupidFloat32 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0)
    }
}
impl MulAssign for StupidFloat32 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self(self.0 * rhs.0)
    }
}
impl DivAssign for StupidFloat32 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self(self.0 / rhs.0)
    }
}
impl RemAssign for StupidFloat32 {
    fn rem_assign(&mut self, rhs: Self) {
        *self = Self(self.0 % rhs.0)
    }
}
