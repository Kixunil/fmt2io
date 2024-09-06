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

Maintenance status
------------------

Passively maintained/done.

The crate does one thing and one thing only.
It works as expected and has sensible API.
It didn't need to be changed for a very long time and only had one significant code change since
its creation.
The other changes were purely for cleanup before 1.0 release.
It's also very small so there's pretty much zero chance of bugs.
Therefore I consider it done.
It's not dead, there just doesn't seem to be anything that needs changing.

MSRV
----

The minimal supported version of Rust is **1.0.0**.
This is not a joke, the crate really doesn't require any fancy compiler features.
However I don't object to raising the MSRV up to what's in Debian stable if some really good
reason arises. This is highly unlikely to happen.

License
-------

MITNFA
