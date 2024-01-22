# Development notes

Since it is not possible (yet) to produce binding from Rust to C++ library, I have take another approach.

1. Create a C++ to C library: `quickfix-ffi/quickfix-bind`.
2. Create a C to Rust unsafe library: `quickfix-ffi`.
3. Create a Rust unsafe to safe library: `quickfix`.

## Main library design

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

## Message generator design

There are few sub-components to make it works:

```txt

📄 FIX XML file (ex: FIX43.xml)
   |
   v
📦 quickfix-spec-parser: parse XML file to a rust struct / enum model.
   |
   v
📦 quickfix-msg-gen: generate code from the XML model.
   |
   v
📦 quickfix-msg43: contains a `build.rs` file to include generated code into a real crate.

```

Few more words on `quickfix-spec-parser`: this crate is agnostic from **any** programming language.
It is a pure representation of the XML spec file as rust struct and enums.
It can be used to generate code, doc, whatever you want to.

## A note about exceptions

Rust `std::error::Error` and C++ exceptions cannot be match together.

Moreover:

- Rust `panic` [should not leak](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html#notes) into C++ code.
- C++ `throw` should not leak into Rust code.

So we have to:

1. Catch **EVERY** exceptions that can occurs in C++ code and convert them to error code.

   - That is the purpose of the C macro: `CATCH_OR_RETURN`.
   - Error code can be converted back to text / code using `Fix_getLastErrorCode` and `Fix_getLastErrorMessage`.

2. Catch **EVERY** `panic` that can occurs in Rust code.

    - For this we are using `std::panic::catch_unwind` to wrap every user callbacks.
    - Sadly we do not cancel the control flow if this occurs.
    - panic message will be displayed on screen but that's all... User can still register a new panic hook if needed.

Wait, what about intentional control flow change (like `DoNotSend`) ?

Here is how it works:

1. User return `MsgToAppError::DoNotSend` from its callback.
2. enum is converted to an integer that will be passed to C code.
3. depending on the integer code, exception will be throw from C++ in `ApplicationBind`.

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
