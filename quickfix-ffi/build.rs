use std::env;

use cmake::Config;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=../CMakeLists.txt");
    println!("cargo:rerun-if-changed=../quickfix-bind/CMakeLists.txt");
    println!("cargo:rerun-if-changed=../quickfix-bind/include/quickfix_bind.h");
    println!("cargo:rerun-if-changed=../quickfix-bind/src/quickfix_bind.cpp");

    // Build quickfix as a static library
    let quickfix_dst = Config::new("../libquickfix")
        .define("HAVE_PYTHON", "OFF")
        .define("HAVE_PYTHON3", "OFF")
        .define("QUICKFIX_SHARED_LIBS", "OFF")
        .define("QUICKFIX_EXAMPLES", "OFF")
        .define("QUICKFIX_TESTS", "OFF")
        .build();

    let quickfix_include_path = format!("{}/include", quickfix_dst.display());
    let quickfix_lib_path = format!("{}/lib", quickfix_dst.display());

    // Build quickfix C bind also as a static library.
    env::set_var("CMAKE_LIBRARY_PATH", [quickfix_lib_path].join(";"));

    let quickfix_bind_dst = Config::new("..")
        .cflag(format!("-I{quickfix_include_path}"))
        .cxxflag(format!("-I{quickfix_include_path}"))
        .define("QUICKFIX_BIND_EXAMPLES", "OFF")
        .build();

    // Configure rustc.
    println!(
        "cargo:rustc-link-search=native={}/lib",
        quickfix_dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib",
        quickfix_bind_dst.display()
    );
    println!("cargo:rustc-link-lib=static=quickfix");
    println!("cargo:rustc-link-lib=static=quickfixbind");
    println!("cargo:rustc-link-lib=stdc++");
}
