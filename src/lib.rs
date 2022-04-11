// SPDX-License-Identifier: Apache-2.0

//! This crate contains utilities for managing CPU primitives like:
//!   * Registers
//!   * Addresses
//!   * Offsets
//!   * Pages

#![no_std]
#![forbid(clippy::expect_used, clippy::panic)]
#![deny(
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications,
    clippy::all,
    missing_docs
)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod address;
mod offset;
mod page;
mod pages;
mod register;

pub use address::Address;
pub use offset::Offset;
pub use page::Page;
pub use pages::Pages;
pub use register::Register;

/// Defines the additive identity value
pub trait Zero: Copy {
    /// Additive identity
    const ZERO: Self;
}

/// Defines the multiplicative identity value
pub trait One: Copy {
    /// Multiplicative identity
    const ONE: Self;
}

macro_rules! impltraits {
    ($($num:ty)+) => {
        $(
            impl Zero for $num {
                const ZERO: Self = 0;
            }

            impl One for $num {
                const ONE: Self = 1;
            }
        )+
    };
}

impltraits! {
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
}
