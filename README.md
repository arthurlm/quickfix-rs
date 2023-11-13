# Quick fix binding for Rust

This project is WIP to allow binding between quickfix library and rust project.

## How to ?

Build C binding library:

    mkdir target
    cd target
    CFLAGS="-I$HOME/.local/include" CXXFLAGS="-I$HOME/.local/include" LDFLAGS="-L$HOME/.local/lib" cmake \
        -DCMAKE_BUILD_TYPE=Release \
        -DQUICKFIX_BIND_EXAMPLES=ON \
        ..
    make

Run C binding example:

    LD_LIBRARY_PATH="$HOME/.local/lib" ./quickfix-bind/demo_basic_binding ../example/settings.ini

Rust FFI example:

    cargo r --example demo_basic_ffi -- example/settings.ini

Run rust full binding example:

    cargo r --example demo_basic -- example/settings.ini

## Build requirements

Following package must be install to build the library:

- `cmake`
- a C++ compiler
- `rustup` / `rustc` / `cargo` (obviously 😉)
