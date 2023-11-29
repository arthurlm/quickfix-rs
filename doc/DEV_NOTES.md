# Development notes

Since it is not possible (yet) to produce binding from Rust to C++ library, I have take another approach.

1. Create a C++ to C library: `quickfix-ffi/quickfix-bind`.
2. Create a C to Rust unsafe library: `quickfix-ffi`.
3. Create a Rust unsafe to safe library: `quickfix`.

## Current design

Actually there is 3 components in the project

```txt

quickfix-ffi/libquickfix (lang: C++)
    ^
    |
    v
quickfix-ffi/quickfix-bind (lang: C)
    ^
    |   ABI=C
    v
quickfix-ffi (lang: rust)
    ^
    |   Wrapper to add safety to `unsafe` bloc.
    v
quickfix (lang: rust)

```

About C++ to C binding library:

- `.cpp` file is made of multiple macros to try making it as short as possible.
- `.h` contains less possible macro to make it easier to compare with rust code.

About compilation process:

- Everything is statically linked in the final binary:

  - `libquickfix` is rebuild from this repository using git sub repo.
  - `libquickfix_bind` is also build from here.

- I have not implement search of existing library installation using `pkg-config`, but PR are welcomed.

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

[Rust cxx](https://github.com/dtolnay/cxx)

I have found this project after starting the project.
However to me it still have some issue.

Pro:

- Better support of C++ STL than bindgen.
- Looks more popular and supported than bindgen.

Cons:

- Still require manual writing of a bridge between Rust / C++ libraries.
  So why adding a library, I could make the bridge from standard C11 and use standard Rust.
  See more details on how it works [here](https://cxx.rs/).

## Other project inspiration

- [rust-rdkafka](https://github.com/fede1024/rust-rdkafka): for how to build C library using cmake and link to rust project.
