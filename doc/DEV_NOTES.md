# Development notes

## Other way to bind C++ library to rust

[Rust bindgen](https://github.com/rust-lang/rust-bindgen)

Pros:

- No C binding library are required.
- All functions of the original library "should" be available.

Cons:

- Does not work for now since many C++ features are not available "correctly".
  - inline function.
  - vtable correct usage.
  - operator overloading.
  - stdlib is not correctly handle.
- Still require an unsafe to safe rust wrapper.
- Output code is just awful and terribly slow to compile.

## Current design

Actually there is 3 components in the project

```txt

quickfix (lang: C++)
    ^
    |
    v
quickfix-bind (lang: C)
    ^
    |   ABI=C
    v
quickfix-ffi (lang: rust)
    ^
    |   Wrapper to add safety to `unsafe` bloc.
    v
quickfix (lang: rust)

```
