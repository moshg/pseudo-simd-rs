#![no_std]

use core::fmt;
use core::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Neg, Not, Sub, SubAssign};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct u7x8(u64);

impl fmt::Debug for u7x8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut dt = f.debug_tuple("u7x8");
        for i in 0..8 {
            dt.field(&((self.0 >> (i * 8)) & 0xFF));
        }
        dt.finish()
    }
}

const MASK: u64 = 0x7F7F_7F7F_7F7F_7F7F;

impl u7x8 {
    /// Constructs a new instance with each element initialized to `value`.
    ///
    /// # Panics
    ///
    /// If `value` is greater than `u7::MAX` (= 127).
    #[inline]
    pub fn splat(value: u8) -> Self {
        assert!(value <= 0x7F);
        let value = value as u64;
        let mut ret = 0;
        for i in 0..8 {
            ret |= value << (i * 8);
        }
        u7x8(ret)
    }
}

impl BitAnd<Self> for u7x8 {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        u7x8(self.0 & rhs.0)
    }
}

impl BitAndAssign<Self> for u7x8 {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl BitOr<Self> for u7x8 {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        u7x8(self.0 | rhs.0)
    }
}

impl BitOrAssign<Self> for u7x8 {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl BitXor<Self> for u7x8 {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        u7x8(self.0 ^ rhs.0)
    }
}

impl BitXorAssign<Self> for u7x8 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: u7x8) {
        *self = *self ^ rhs
    }
}

impl Add<Self> for u7x8 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        u7x8((self.0 + rhs.0) & MASK)
    }
}

impl AddAssign<Self> for u7x8 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Not for u7x8 {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        u7x8(!self.0 & MASK)
    }
}

impl Neg for u7x8 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        !self + u7x8::splat(1)
    }
}

impl Sub<Self> for u7x8 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        // The inner addition won't overflow u8 because u7::MAX + u7::MAX + 1 == u8::MAX
        u7x8((self.0 + (!rhs).0 + u7x8::splat(1).0) & MASK)
    }
}

impl SubAssign<Self> for u7x8 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

#[cfg(test)]
mod tests {
    use crate::u7x8;

    #[test]
    fn test_splat() {
        let a = u7x8::splat(1);
        assert_eq!(a, u7x8(0x101_0101_0101_0101));
    }

    #[test]
    fn test_add() {
        let a = u7x8::splat(0x7F);
        let b = u7x8::splat(1);
        assert_eq!(a + b, u7x8::splat(0));
    }

    #[test]
    fn test_sub() {
        let a = u7x8::splat(0x7F);
        let b = u7x8::splat(0);
        assert_eq!(a - a, u7x8::splat(0));
        assert_eq!(a - b, a);
    }
}
