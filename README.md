# Quick fix binding for Rust

This project is WIP to allow binding between quickfix library and rust project.

To build:

    mkdir target
    cd target
    cmake -DCMAKE_BUILD_TYPE=Release ..
    make
    ./quickfix-bind/quickfix_bind_tester ../example/settings.ini

To run rust FFI example:

    LD_LIBRARY_PATH=$(pwd)/target/quickfix-bind cargo r -- example/settings.ini
