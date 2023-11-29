# How do I ?

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
