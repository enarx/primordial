// SPDX-License-Identifier: Apache-2.0

use super::*;
use core::cmp::Ordering;
use core::marker::PhantomData;
use core::mem::size_of;
use core::ops::*;

/// An offset of a number of items of type `T` from a base
///
/// Note well that this is NOT stored in memory as the number of bytes,
/// but rather the number of items.
///
/// One important additional feature is that offsets can be converted between
/// underlying types so long as the conversion is lossless for the target CPU
/// architecture. For example, `Offset<u64>` can be converted to
/// `Offset<usize>` on 64-bit systems.
#[derive(Copy, Clone, Default)]
#[repr(transparent)]
pub struct Offset<T, U>(T, PhantomData<U>);

impl<T: core::fmt::Debug, U> core::fmt::Debug for Offset<T, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.0, f)
    }
}

impl<T: core::fmt::Binary, U> core::fmt::Binary for Offset<T, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Binary::fmt(&self.0, f)
    }
}

impl<T: core::fmt::Display, U> core::fmt::Display for Offset<T, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}

impl<T: core::fmt::LowerHex, U> core::fmt::LowerHex for Offset<T, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::LowerHex::fmt(&self.0, f)
    }
}

impl<T: core::fmt::UpperHex, U> core::fmt::UpperHex for Offset<T, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::UpperHex::fmt(&self.0, f)
    }
}

impl<T: PartialEq, U> PartialEq for Offset<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Eq, U> Eq for Offset<T, U> {}

impl<T: PartialOrd, U> PartialOrd for Offset<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Ord, U> Ord for Offset<T, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

#[cfg(feature = "const-default")]
impl<T: Zero, U> const_default::ConstDefault for Offset<T, U> {
    const DEFAULT: Self = Self(T::ZERO, PhantomData);
}

impl<T, U> Offset<T, U> {
    /// Create an offset value from the number of items
    #[inline]
    pub const fn from_items(items: T) -> Self {
        Self(items, PhantomData)
    }

    /// Get the number of items
    #[inline]
    pub fn items(self) -> T {
        self.0
    }
}

impl<T, U> Offset<T, U>
where
    Offset<usize, ()>: Into<Offset<T, ()>>,
    T: Mul<T, Output = T>,
{
    /// Get the number of bytes
    #[inline]
    pub fn bytes(self) -> T {
        self.0 * Offset(size_of::<U>(), PhantomData).into().items()
    }
}

impl<T: Zero, U: Copy> Zero for Offset<T, U> {
    const ZERO: Offset<T, U> = Offset::from_items(T::ZERO);
}

impl<T: One, U: Copy> One for Offset<T, U> {
    const ONE: Offset<T, U> = Offset::from_items(T::ONE);
}

impl<T: From<Register<T>>, U> From<Register<T>> for Offset<T, U> {
    #[inline]
    fn from(value: Register<T>) -> Self {
        Self::from_items(T::from(value))
    }
}

impl<T, U> From<Offset<T, U>> for Register<T>
where
    Register<T>: From<T>,
{
    #[inline]
    fn from(value: Offset<T, U>) -> Self {
        Self::from(value.0)
    }
}

#[cfg(target_pointer_width = "64")]
impl<U> From<Offset<u64, U>> for Offset<usize, U> {
    #[inline]
    fn from(value: Offset<u64, U>) -> Self {
        Self(value.0 as _, PhantomData)
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl<U> From<Offset<usize, U>> for Offset<u64, U> {
    #[inline]
    fn from(value: Offset<usize, U>) -> Self {
        Self(value.0 as _, PhantomData)
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl<U> From<Offset<u32, U>> for Offset<usize, U> {
    #[inline]
    fn from(value: Offset<u32, U>) -> Self {
        Self(value.0 as _, PhantomData)
    }
}

#[cfg(target_pointer_width = "32")]
impl<U> From<Offset<usize, U>> for Offset<u32, U> {
    #[inline]
    fn from(value: Offset<usize, U>) -> Self {
        Self(value.0 as _, PhantomData)
    }
}

impl<T: Add<T, Output = T>, U> Add for Offset<T, U> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, PhantomData)
    }
}

impl<T: AddAssign<T>, U> AddAssign for Offset<T, U> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<T: Div<T, Output = T>, U> Div for Offset<T, U> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0, PhantomData)
    }
}

impl<T: DivAssign<T>, U> DivAssign for Offset<T, U> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl<T: Mul<T, Output = T>, U> Mul for Offset<T, U> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, PhantomData)
    }
}

impl<T: MulAssign<T>, U> MulAssign for Offset<T, U> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl<T: Rem<T, Output = T>, U> Rem for Offset<T, U> {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0, PhantomData)
    }
}

impl<T: RemAssign<T>, U> RemAssign for Offset<T, U> {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0;
    }
}

impl<T: Sub<T, Output = T>, U> Sub for Offset<T, U> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, PhantomData)
    }
}

impl<T: SubAssign<T>, U> SubAssign for Offset<T, U> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
