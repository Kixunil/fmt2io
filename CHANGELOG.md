# Changes

## 1.0.0 - Fri Sep 6 2024

* The crate is now considered stable
* Support Rust 1.0.0
* Various documentation improvements
* Fixed the `maintenance` badge

## 0.2.0 - Tue Jun 22 2021

Breaking change: the `write` function can now return any value in the `Ok` variant.
This is breaking because the number of generic arguments changed,
however if you didn't use turbofish to specify them there's no real break for you.

## 0.1.0 - Tue Nov 6 2018

Initial version capable of bridging `fmt` to `io` writes that didn't produce any value.
