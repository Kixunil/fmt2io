//! Have you ever implemented a nice algorithm that generically uses [`fmt::Write`](std::fmt::Write)
//! only to find out it doesn't work with [`io::Write`](std::io::Write)? Worry no more - this is the
//! solution!
//! 
//! This crate provides a simple `write` function which takes your [`io::Write`](std::io::Write)r,
//! converts it to [`fmt::Write`](std::fmt::Write)r and provides it to your closure. This way, you
//! can easily bridge the two traits and have truly generic code.
//!
//! Example
//! -------
//!
//! ```rust
//! let mut out = Vec::new();
//!
//! use std::fmt::Write;
//!
//! fmt2io::write(&mut out, |writer| write!(writer, "Hello world!"))?;
//! assert_eq!(out, "Hello world!".as_bytes());
//!
//! # Ok::<(), std::io::Error>(())
//! ```

extern crate fmt2io;

pub use fmt2io::*;
