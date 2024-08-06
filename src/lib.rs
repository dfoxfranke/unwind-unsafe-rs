// Copyright 2024 Daniel Fox Franke
// SPDX-License-Identifier: MIT-0
#![no_std]
#![forbid(unsafe_code)]

//! This crate provides zero-sized marker types which do not implement
//! [`UnwindSafe`] or [`RefUnwindSafe`]. They can be added to types to prevent
//! the respective auto traits from being derived.
//!
//! These types can be instantiated via their [`Default`] implementation or from
//! their associated constant `DEFAULT`.
//!
//! This crate has no dependencies, no unsafe code, and is `no_std` by default.

use core::fmt::Debug;
use core::panic::{RefUnwindSafe, UnwindSafe};

#[allow(unused)]
trait NoImplementers {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct NotUwsInner<T>(T);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct NotRuwsInner<T>(T);

impl<T> UnwindSafe for NotUwsInner<T> where T: NoImplementers {}
impl<T> RefUnwindSafe for NotRuwsInner<T> where T: NoImplementers {}

/// A marker type which does not implement [`UnwindSafe`].
///
/// If a type contains a `PhantomUnwindUnsafe`, it will not implement
/// `UnwindSafe` by default.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PhantomUnwindUnsafe(NotUwsInner<()>);

/// A marker type which does not implement [`RefUnwindSafe`].
///
/// If a type contains a `PhantomRefUnwindUnsafe`, it will not implement
/// `RefUnwindSafe` by default.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PhantomRefUnwindUnsafe(NotRuwsInner<()>);

impl PhantomUnwindUnsafe {
    pub const DEFAULT: Self = Self(NotUwsInner(()));
}

impl PhantomRefUnwindUnsafe {
    pub const DEFAULT: Self = Self(NotRuwsInner(()));
}

impl Debug for PhantomUnwindUnsafe {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("PhantomUnwindUnsafe")
    }
}

impl Debug for PhantomRefUnwindUnsafe {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("PhantomRefUnwindUnsafe")
    }
}
