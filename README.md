# Quick fix binding for Rust

This project is an **unofficial** binding between [quickfix library](https://github.com/quickfix/quickfix) and rust project.

## What is it, what it is not ?

Main idea of this project is to provide a **minimal** safe and working bridge between rust and C++ quickfix library.

Not all methods, functions, classes, nor features of the original library will be exposed to Rust crate.
Target is just to have minimal features to create small and safe applications like:

- some program that can send order / receive messages to adjust strategy from.
- basic router with no smart order router capabilities.

## Is it ready for production ?

Yes. But keep in mind that not every feature of the original FIX library are available.
If some of your needs are missing: PR / feedbacks are welcomed 😁!

## How does it work ?

Since it is not possible (yet) to produce binding from Rust to C++ library, I have take another approach.

1. Create a C++ to C library: `quickfix-bind`.
2. Create a C to Rust unsafe library: `quickfix-ffi`.
3. Create a Rust unsafe to safe library: `quickfix`.

Check [DEV_NOTES](./doc/DEV_NOTES.md) for more information on the dev workflow and my research.

## How do I ?

Build C binding library:

```sh
mkdir target
cd target
CFLAGS="-I$HOME/.local/include" CXXFLAGS="-I$HOME/.local/include" LDFLAGS="-L$HOME/.local/lib" cmake \
    -DCMAKE_BUILD_TYPE=Release \
    -DQUICKFIX_BIND_EXAMPLES=ON \
    ..
make
```

Run C binding example:

```sh
LD_LIBRARY_PATH="$HOME/.local/lib" ./quickfix-bind/demo_basic_binding ../example/settings.ini
```

Rust FFI example:

```sh
cargo r --example demo_basic_ffi -- example/settings.ini
```

Run rust full binding example:

```sh
cargo r --example fix_getting_started -- example/server.ini
cargo r --example fix_repl -- acceptor example/server.ini
cargo r --example fix_repl -- initiator example/client.ini
```

## Build requirements

Following package must be install to build the library:

- `cmake`
- a C++ compiler (with C++17 support)
- `rustup` / `rustc` / `cargo` (obviously 😉)
