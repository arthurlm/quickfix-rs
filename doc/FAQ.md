# Frequently Asked Questions

## How do I ?

Build C binding library:

```sh
mkdir build
cd build
CFLAGS="-I$HOME/.local/include" CXXFLAGS="-I$HOME/.local/include" LDFLAGS="-L$HOME/.local/lib" cmake \
    -DCMAKE_BUILD_TYPE=Release \
    -DQUICKFIX_BIND_EXAMPLES=ON \
    ../quickfix-ffi
make
```

Run C binding example:

```sh
LD_LIBRARY_PATH="$HOME/.local/lib" ./examples/demo_basic_binding ../configs/settings.ini
```

Rust FFI example:

```sh
cargo r --example demo_basic_ffi -- configs/settings.ini
```

Run rust full binding example:

```sh
cargo r --example fix_getting_started -- configs/server.ini
cargo r --example fix_repl -- acceptor configs/server.ini
cargo r --example fix_repl -- initiator configs/client.ini
```

## What is the rust library overhead vs C++ quickfix

I have not benchmark it.

I have try to design rust wrapper with the minimal footprint as possible.
My guess is (if [LTO](https://doc.rust-lang.org/cargo/reference/profiles.html#lto) is enabled) overhead will be pretty low.

For example, if we look at `quickfix::Message`:

- it is a wrapper of `quickfix_ffi::FixMessage_t`
- which is a wrapper of `std::ptr::NonNull<ffi::c_void>`
- which will be used in `quickfix-bind` library to directly call quickfix c++ functions.

Moreover you can check `quickfix::Message` size is the size of a pointer.
