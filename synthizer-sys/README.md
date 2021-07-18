# Low-level Synthizer bindings

![CI Status](https://github.com/github/synthizer/synthizer-rs/workflows/main.yml/badge.svg)
[GitHub Sponsors](https://github.com/sponsors/ahicks92)

This crate contains low-level Synthizer bindings generated with bindgen, plus a
vendored copy of Synthizer.  Currently, it is impossible to use dynamically
linked/system versions of Synthizer, but this restriction will likely ebe lifted
with Synthizer 1.0 when proper packaging eventually exists.

Synthizer itself is always compiled in release mode even in debug builds due to
limitations in the cmake crate and the inability to change which version of the
C standard library Rust links to on Windows.  If you know how to fix this, feel
free to open a  PR.