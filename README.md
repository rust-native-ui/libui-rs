# libUI
[![libui-rs build status](https://api.travis-ci.org/LeoTindall/libui-rs.svg?branch=master)](https://travis-ci.org/LeoTindall/libui-rs/)

A Rusty user interface library that binds to platform native APIs. 
These are work-in-progress Rust bindings to the minimalistic native UI library [libui][libui].

## Building
`libui` is included as a submodule. You will need CMake to build `libui` itself; 
after that, Cargo should be able to take care of the build process.

Based on work by @pcwalton

[libui]: https://github.com/andlabs/libui

# Testing Note

Travis does not connect video devices to their testing environments, so the tests cannot be run. Therefore, the "tests" only check compilation.
