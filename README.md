Fmt to IO
=========

A bridge between `std::io::Write` and `std::fmt::Write`.

About
-----

Have you ever implemented a nice algorithm that generically uses `fmt::Write`
only to find out it doesn't work with `io::Write`? Worry no more - this is the
solution!

This crate provides a simple `write` function which takes your `io::Write`r,
converts it to `fmt::Write`r and provides it to your closure. This way, you
can easily bridge the two traits and have truly generic code.

License
-------

MITNFA
