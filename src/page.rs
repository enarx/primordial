// SPDX-License-Identifier: Apache-2.0

use core::borrow::{Borrow, BorrowMut};
use core::ops::{Deref, DerefMut};

/// A single page of memory
///
/// This type is page-aligned and page-sized.
#[derive(Copy, Clone)]
#[repr(C, align(4096))]
pub struct Page([u8; Self::SIZE]);

#[cfg(feature = "const-default")]
impl const_default::ConstDefault for Page {
    const DEFAULT: Self = Self::zeroed();
}

impl Default for Page {
    #[inline]
    fn default() -> Self {
        Self::zeroed()
    }
}

impl Deref for Page {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.align_to().1 }
    }
}

impl DerefMut for Page {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.align_to_mut().1 }
    }
}

impl AsRef<[u8]> for Page {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        unsafe { self.0.align_to().1 }
    }
}

impl AsMut<[u8]> for Page {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { self.0.align_to_mut().1 }
    }
}

impl Borrow<[u8]> for Page {
    #[inline]
    fn borrow(&self) -> &[u8] {
        unsafe { self.0.align_to().1 }
    }
}

impl BorrowMut<[u8]> for Page {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [u8] {
        unsafe { self.0.align_to_mut().1 }
    }
}

impl Page {
    /// The page size on the platform
    pub const SIZE: usize = 4096;

    /// Creates a new page from its bytes
    #[inline]
    pub const fn new(value: [u8; Self::SIZE]) -> Self {
        Self(value)
    }

    /// Returns a Page full of zeroes
    pub const fn zeroed() -> Self {
        Self([0; Self::SIZE])
    }
}
