#![allow(missing_docs)]

use super::Page;

use core::borrow::{Borrow, BorrowMut};
use core::ops::{Deref, DerefMut};

/// A wrapper type around types that provide page slices
pub struct Pages<T>(T);

impl<T> Pages<T> {
    /// Wraps the specified value
    #[inline]
    pub const fn new(value: T) -> Self {
        Self(value)
    }
}

#[cfg(feature = "const-default")]
impl<const N: usize> const_default::ConstDefault for Pages<[Page; N]> {
    const DEFAULT: Self = Self([Page::DEFAULT; N]);
}

#[cfg(feature = "alloc")]
impl Pages<alloc::vec::Vec<Page>> {
    /// Copies all specified bytes into a page-aligned vector
    pub fn copy(data: &[u8]) -> Self {
        Self::copy_into(data, data.len(), 0)
    }

    /// Copyies some bytes into a page-aligned vector at an offset
    ///
    /// This function allocates a zeroed page buffer big enough to hold
    /// `offset + size` bytes. Then up to `size` bytes of `data` are copied
    /// into the buffer at the specified `offset`. Note that `size` may be
    /// larger than `data.len()` in order to allocate a bigger buffer.
    pub fn copy_into(data: &[u8], size: usize, offset: usize) -> Self {
        let data = &data[..core::cmp::min(size, data.len())];

        // Allocate a buffer large enough for offset + size.
        let count = (offset + size + Page::SIZE - 1) / Page::SIZE;
        let mut buf = alloc::vec::Vec::with_capacity(count);
        let bytes: &mut [u8] = unsafe {
            buf.set_len(count);
            buf.align_to_mut().1
        };

        // Segment the regions.
        let (prefix, bytes) = bytes.split_at_mut(offset);
        let (bytes, suffix) = bytes.split_at_mut(data.len());

        // Copy and zero.
        prefix.fill(0);
        bytes.copy_from_slice(data);
        suffix.fill(0);

        Self(buf)
    }
}

impl<T> From<T> for Pages<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Deref<Target = [Page]>> Deref for Pages<T> {
    type Target = [Page];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T: DerefMut<Target = [Page]>> DerefMut for Pages<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

impl<T: AsRef<[Page]>> AsRef<[Page]> for Pages<T> {
    #[inline]
    fn as_ref(&self) -> &[Page] {
        self.0.as_ref()
    }
}

impl<T: AsMut<[Page]>> AsMut<[Page]> for Pages<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [Page] {
        self.0.as_mut()
    }
}

impl<T: AsRef<[Page]>> AsRef<[u8]> for Pages<T> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        unsafe { self.0.as_ref().align_to().1 }
    }
}

impl<T: AsMut<[Page]>> AsMut<[u8]> for Pages<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { self.0.as_mut().align_to_mut().1 }
    }
}

impl<T: Borrow<[Page]>> Borrow<[Page]> for Pages<T> {
    #[inline]
    fn borrow(&self) -> &[Page] {
        self.0.borrow()
    }
}

impl<T: BorrowMut<[Page]>> BorrowMut<[Page]> for Pages<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [Page] {
        self.0.borrow_mut()
    }
}

impl<T: Borrow<[Page]>> Borrow<[u8]> for Pages<T> {
    #[inline]
    fn borrow(&self) -> &[u8] {
        unsafe { self.0.borrow().align_to().1 }
    }
}

impl<T: BorrowMut<[Page]>> BorrowMut<[u8]> for Pages<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [u8] {
        unsafe { self.0.borrow_mut().align_to_mut().1 }
    }
}
