//! Have you ever implemented a nice algorithm that generically uses `fmt::Write`
//! only to find out it doesn't work with `io::Write`? Worry no more - this is the
//! solution!
//! 
//! This crate provides a simple `write` function which takes your `io::Write`r,
//! converts it to `fmt::Write`r and provides it to your closure. This way, you
//! can easily bridge the two traits and have truly generic code.
//!
//! Example
//! -------
//!
//! ```rust
//! use std::fmt::Write;
//!
//! let mut out = Vec::new();
//!
//! fmt2io::write(&mut out, |writer| write!(writer, "Hello world!"))?;
//! assert_eq!(out, "Hello world!".as_bytes());
//!
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! MSRV
//! ----
//!
//! The minimal supported version of Rust is **1.0.0**.
//! This is not a joke, the crate really doesn't require any fancy compiler features.
//! However I don't object to raising the MSRV up to what's in Debian stable if some really good
//! reason arises. This is highly unlikely to happen.

use std::{fmt, io};

/// Converts given [`io::Write`]r to [`fmt::Write`]r.
///
/// The [`Writer`] is constructed from your `writer` which you
/// can use to write UTF-8 data to. The function returns underlying io
/// Error, if there has been one.
///
/// This function uses closure instead of directly exposing [`Writer`]
/// in order to make error handling ergonomic/idiomatic.
///
/// See the [crate-level documentation](crate) for an example.
///
/// ## Panics
///
/// This function panics if `writer` didn't return an error but you
/// return `Err` from the closure. Return `Ok(Err(YourError))` to handle
/// your own error cases.
pub fn write<R, W, F>(writer: W, f: F) -> io::Result<R> where W: io::Write, F: FnOnce(&mut Writer<W>) -> Result<R, fmt::Error> {
    let mut writer = Writer {
        writer: writer,
        result: Ok(()),
    };

    let result = f(&mut writer);

    writer.result.map(move |_| result.unwrap())
}

/// A bridge between [`std::io::Write`] and [`std::fmt::Write`].
///
/// This struct provides [`fmt::Write`] implementation for inner
/// writers implementing [`io::Write`]. It must be used within the
/// [`write()`] function.
///
/// See the documentation of [`write()`] for more information.
#[derive(Debug)]
pub struct Writer<W: io::Write> {
    writer: W,
    result: Result<(), io::Error>,
}

impl<W: io::Write> fmt::Write for Writer<W> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.writer.write_all(s.as_bytes()).map_err(|err| { self.result = Err(err); fmt::Error })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use std::fmt::Write;

        let mut out = Vec::new();

        ::write(&mut out, |writer| write!(writer, "Hello world!")).unwrap();

        assert_eq!(out, "Hello world!".as_bytes());
    }
}
