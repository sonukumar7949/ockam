//! Core types of the Ockam library.
//!
//! This crate contains the core types of the Ockam library and is intended
//! for use by other crates that provide features and add-ons to the main
//! Ockam library.
//!
//! The main Ockam crate re-exports types defined in this crate.
#![deny(unsafe_code)]
#![warn(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), not(feature = "alloc")))]
compile_error!(r#"The "no_std" feature currently requires the "alloc" feature"#);

#[cfg(feature = "std")]
extern crate core;

#[cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;

// Allow use of logging macros directly.
#[macro_use]
extern crate tracing;

pub use async_trait::async_trait;

#[allow(unused_imports)]
#[macro_use]
/// Re-export of the `async_trait` macro crate.
pub extern crate async_trait;

/// Mark an Ockam Worker implementation.
#[doc(inline)]
pub use async_trait::async_trait as worker;
/// Mark an Ockam Processor implementation.
#[doc(inline)]
pub use async_trait::async_trait as processor;

extern crate ockam_macros;
pub use ockam_macros::{AsyncTryClone, Message};

extern crate futures_util;

mod access_control;
pub mod compat;
pub mod error;
mod message;
mod processor;
mod routing;
pub mod traits;
mod uint;
pub mod vault;
mod worker;

pub use crate::error::Result;
pub use access_control::*;
pub use message::*;
pub use processor::*;
pub use routing::*;
pub use traits::*;
pub use uint::*;
pub use worker::*;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
#[doc(hidden)]
pub use compat::println;

pub use thiserror;

#[cfg(feature = "std")]
#[doc(hidden)]
pub use std::println;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
/// println macro for no_std
pub mod println_no_std {
    #[macro_export]
    /// implements println for no_std by wrapping the tracing::info! macro
    macro_rules! println {
        ($($arg:tt)*) => {{
            tracing::info!($($arg)*);
        }};
    }
}

use crate::compat::boxed::Box;

/// Clone trait for async structs.
#[async_trait]
pub trait AsyncTryClone: Sized {
    /// Try cloning a object and return an `Err` in case of failure.
    async fn async_try_clone(&self) -> Result<Self>;
}

#[async_trait]
impl<D> AsyncTryClone for D
where
    D: Clone + Sync,
{
    async fn async_try_clone(&self) -> Result<Self> {
        Ok(self.clone())
    }
}
