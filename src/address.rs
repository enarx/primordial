// SPDX-License-Identifier: Apache-2.0

use super::*;
use core::cmp::Ordering;
use core::marker::PhantomData;
use core::mem::{align_of, size_of};
use core::{ops::*, panic};

/// An address
///
/// This newtype is used to represent addresses of a given type.
/// The most important invariant of this type is that the address is always
/// properly aligned for the given type `U`. The only way to convert between
/// addresses of different types is to choose a new alignment (raise or lower).
///
/// This type does *not*, however, track lifetime. You're on your own.
///
/// Unlike the naked underlying types, you can infallibly convert between,
/// for example, an `Address<usize, ()>` and an `Address<u64, ()>` wherever
/// such a conversion is lossless given the target CPU architecture.
#[derive(Copy, Clone, Default)]
#[repr(transparent)]
pub struct Address<T, U>(T, PhantomData<U>);

impl<T: core::fmt::Binary, U> core::fmt::Binary for Address<T, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Binary::fmt(&self.0, f)
    }
}

impl<T: core::fmt::LowerHex, U> core::fmt::Debug for Address<T, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "Address(0x{:01$x})",
            self.0,
            size_of::<T>() * 2
        ))
    }
}

impl<T: core::fmt::Display, U> core::fmt::Display for Address<T, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}

impl<T: core::fmt::LowerHex, U> core::fmt::LowerHex for Address<T, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::LowerHex::fmt(&self.0, f)
    }
}

impl<T, U> core::fmt::Pointer for Address<T, U>
where
    Self: Into<Address<usize, U>>,
    Self: Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}

impl<T: core::fmt::UpperHex, U> core::fmt::UpperHex for Address<T, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::UpperHex::fmt(&self.0, f)
    }
}

impl<T: PartialEq, U> PartialEq for Address<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Eq, U> Eq for Address<T, U> {}

impl<T: PartialOrd, U> PartialOrd for Address<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Ord, U> Ord for Address<T, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

#[cfg(feature = "const-default")]
impl<T: Zero, U> const_default::ConstDefault for Address<T, U> {
    const DEFAULT: Self = Self::NULL;
}

impl<T: Zero, U> Address<T, U> {
    /// The NULL address
    pub const NULL: Address<T, U> = Address(T::ZERO, PhantomData);
}

impl<U> Address<usize, U> {
    /// Creates a new address
    ///
    /// Panics if the value is not properly aligned.
    #[inline]
    pub const fn new(value: usize) -> Self {
        if value % align_of::<U>() != 0 {
            panic!("unaligned address value");
        }

        Self(value, PhantomData)
    }
}

impl<T, U> Address<T, U> {
    /// Creates a new `Address` from a raw inner type without checking
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not enforce the main constraint
    /// of this type that the address stored is properly aligned to the type.
    ///
    /// For a safe version of this constructor, first create an `Address<T, ()>`
    /// from the raw value and then align to the type you want.
    #[inline]
    pub const unsafe fn unchecked(value: T) -> Self {
        Self(value, PhantomData)
    }

    /// Converts an `Address` to its raw inner type
    #[inline]
    pub fn raw(self) -> T {
        self.0
    }
}

impl<T, U> Address<T, U>
where
    Self: Into<Address<usize, U>>,
{
    /// Returns a raw pointer to its inner type
    ///
    /// # Safety
    /// Behavior is undefined, if the pointer is used and
    /// is not aligned or points to uninitialized memory.
    #[inline]
    pub fn as_ptr(self) -> *const U {
        self.into().0 as *const U
    }

    /// Returns a raw pointer to its inner type
    ///
    /// # Safety
    /// Behavior is undefined, if the pointer is used and
    /// is not aligned or points to uninitialized memory.
    #[inline]
    pub fn as_mut_ptr(self) -> *mut U {
        self.into().0 as *mut U
    }
}

pub struct AlignmentError;

impl<T, U> Address<T, U>
where
    Self: Into<Address<usize, U>>,
    Self: From<Address<usize, U>>,
{
    /// Try casting an existing `Address` into an `Address` of a different type
    ///
    /// Succeeds only, if they have compatible alignment
    #[inline]
    pub fn try_cast<V>(self) -> Result<Address<T, V>, AlignmentError> {
        let addr = self.into();

        if addr.0 % align_of::<V>() != 0 {
            return Err(AlignmentError);
        }

        Ok(Address(Self::from(addr).0, PhantomData))
    }
}

impl<T, U> Address<T, U>
where
    Offset<usize, ()>: Into<Offset<T, ()>>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
    T: Div<T, Output = T>,
    T: One,
{
    /// Cast an existing `Address` into an `Address` of a different type by aligning up
    #[inline]
    pub fn raise<V>(self) -> Address<T, V> {
        let align: T = Offset::from_items(align_of::<V>()).into().items();
        Address((self.0 + align - T::ONE) / align * align, PhantomData)
    }

    /// Cast an existing `Address` into an `Address` of a different type by aligning down
    #[inline]
    pub fn lower<V>(self) -> Address<T, V> {
        let align: T = Offset::from_items(align_of::<V>()).into().items();
        Address(self.0 / align * align, PhantomData)
    }
}

/// Convert a raw address value to an untyped `Address`
impl<T> From<T> for Address<T, ()> {
    #[inline]
    fn from(value: T) -> Self {
        Self(value, PhantomData)
    }
}

/// Convert a reference to an `Address` with the same type
impl<T, U> From<&U> for Address<T, U>
where
    Address<usize, U>: Into<Address<T, U>>,
{
    #[inline]
    fn from(value: &U) -> Self {
        Address(value as *const U as usize, PhantomData).into()
    }
}

/// Convert a mutable pointer to an `Address` with the same type
impl<T, U> From<*mut U> for Address<T, U>
where
    Address<usize, U>: Into<Address<T, U>>,
{
    #[inline]
    fn from(value: *mut U) -> Self {
        Address(value as usize, PhantomData).into()
    }
}

/// Convert a const pointer to an `Address` with the same type
impl<T, U> From<*const U> for Address<T, U>
where
    Address<usize, U>: Into<Address<T, U>>,
{
    #[inline]
    fn from(value: *const U) -> Self {
        Address(value as usize, PhantomData).into()
    }
}

// Convert from a `Register` to an untyped `Address`.
impl<T: From<Register<T>>> From<Register<T>> for Address<T, ()> {
    #[inline]
    fn from(value: Register<T>) -> Self {
        Self::from(T::from(value))
    }
}

// Convert from an `Address` to a `Register`, discarding type.
impl<T, U> From<Address<T, U>> for Register<T>
where
    Register<T>: From<T>,
{
    #[inline]
    fn from(value: Address<T, U>) -> Self {
        Self::from(value.0)
    }
}

#[cfg(target_pointer_width = "64")]
impl<U> From<Address<u64, U>> for Address<usize, U> {
    #[inline]
    fn from(value: Address<u64, U>) -> Self {
        Self(value.0 as _, PhantomData)
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl<U> From<Address<usize, U>> for Address<u64, U> {
    #[inline]
    fn from(value: Address<usize, U>) -> Self {
        Self(value.0 as _, PhantomData)
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl<U> From<Address<u32, U>> for Address<usize, U> {
    #[inline]
    fn from(value: Address<u32, U>) -> Self {
        Self(value.0 as _, PhantomData)
    }
}

#[cfg(target_pointer_width = "32")]
impl<U> From<Address<usize, U>> for Address<u32, U> {
    #[inline]
    fn from(value: Address<usize, U>) -> Self {
        Self(value.0 as _, PhantomData)
    }
}

impl<T, U> Add<Offset<T, U>> for Address<T, U>
where
    Offset<usize, ()>: Into<Offset<T, ()>>,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Offset<T, U>) -> Self::Output {
        Self(self.0 + rhs.bytes(), PhantomData)
    }
}

impl<T, U> AddAssign<Offset<T, U>> for Address<T, U>
where
    Offset<usize, ()>: Into<Offset<T, ()>>,
    T: Mul<T, Output = T>,
    T: AddAssign<T>,
{
    #[inline]
    fn add_assign(&mut self, rhs: Offset<T, U>) {
        self.0 += rhs.bytes();
    }
}

impl<T, U> Sub<Address<T, U>> for Address<T, U>
where
    Offset<usize, ()>: Into<Offset<T, ()>>,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Div<T, Output = T>,
    T: One,
{
    type Output = Offset<T, U>;

    #[inline]
    fn sub(self, rhs: Address<T, U>) -> Self::Output {
        let offset: Offset<T, U> = Offset::from_items(T::ONE);
        Offset::from_items((self.0 - rhs.0) / offset.bytes())
    }
}

impl<T, U> Sub<Offset<T, U>> for Address<T, U>
where
    Offset<usize, ()>: Into<Offset<T, ()>>,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Offset<T, U>) -> Self::Output {
        Self(self.0 - rhs.bytes(), PhantomData)
    }
}

impl<T, U> SubAssign<Offset<T, U>> for Address<T, U>
where
    Offset<usize, ()>: Into<Offset<T, ()>>,
    T: Mul<T, Output = T>,
    T: SubAssign<T>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Offset<T, U>) {
        self.0 -= rhs.bytes();
    }
}

#[cfg(test)]
mod test {
    extern crate std;

    use super::*;
    use std::println;

    #[test]
    fn align() {
        assert_eq!(Address::from(9usize).raise::<u64>().raw(), 16);
        assert_eq!(Address::from(9usize).lower::<u64>().raw(), 8);
        assert_eq!(Address::from(7usize).raise::<u32>().raw(), 8);
        assert_eq!(Address::from(7usize).lower::<u32>().raw(), 4);
    }

    #[test]
    fn print_pointer() {
        println!("{:p}", Address::from(4usize).raise::<Page>());
        println!("{:p}", Address::from(7u64).lower::<u32>());
    }
}
