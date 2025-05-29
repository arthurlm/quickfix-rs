use std::{env, fs, path::Path, process::Command};

use cmake::Config;
use fs_extra::dir::CopyOptions;

fn have_feature(flag: &str) -> bool {
    env::var(format!(
        "CARGO_FEATURE_{}",
        flag.replace('-', "_").to_uppercase()
    ))
    .is_ok()
}

fn read_cmake_opt(flag: &str) -> &'static str {
    if have_feature(flag) {
        "ON"
    } else {
        "OFF"
    }
}

fn touch_file<P: AsRef<Path>>(path: P) {
    fs::write(path, "").expect("Fail to touch file");
}

fn main() {
    let out_dir = env::var("OUT_DIR").expect("Missing OUT_DIR");
    let target_os = TargetOs::from_env();

    // Make sure sub-repositories are correctly init
    update_sub_repositories();

    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=./CMakeLists.txt");
    println!("cargo:rerun-if-changed=./libquickfix");
    println!("cargo:rerun-if-changed=./quickfix-bind");

    // Clone libquickfix to OUT_DIR because it modify itself when building
    let libquickfix_build_dir = Path::new(&out_dir).join("libquickfix");

    let _ = fs::remove_dir_all(&libquickfix_build_dir);
    fs_extra::copy_items(&["./libquickfix"], &out_dir, &CopyOptions::default())
        .expect("Fail to copy libquickfix");

    // Inject stubs files
    let libquickfix_cpp_dir: std::path::PathBuf = libquickfix_build_dir.join("src/C++");
    touch_file(libquickfix_cpp_dir.join("SSLSocketAcceptor.h"));
    touch_file(libquickfix_cpp_dir.join("SSLSocketConnection.h"));
    touch_file(libquickfix_cpp_dir.join("SSLSocketInitiator.h"));
    touch_file(libquickfix_cpp_dir.join("ThreadedSSLSocketAcceptor.h"));
    touch_file(libquickfix_cpp_dir.join("ThreadedSSLSocketConnection.h"));
    touch_file(libquickfix_cpp_dir.join("ThreadedSSLSocketInitiator.h"));
    touch_file(libquickfix_cpp_dir.join("UtilitySSL.h"));

    touch_file(libquickfix_cpp_dir.join("ThreadedSocketAcceptor.h"));
    touch_file(libquickfix_cpp_dir.join("ThreadedSocketAcceptor.cpp"));
    touch_file(libquickfix_cpp_dir.join("ThreadedSocketConnection.h"));
    touch_file(libquickfix_cpp_dir.join("ThreadedSocketConnection.cpp"));
    touch_file(libquickfix_cpp_dir.join("ThreadedSocketInitiator.h"));
    touch_file(libquickfix_cpp_dir.join("ThreadedSocketInitiator.cpp"));

    // Build quickfix as a static library
    let quickfix_dst = Config::new(libquickfix_build_dir)
        .define("CMAKE_POLICY_VERSION_MINIMUM", "3.10")
        .define("HAVE_SSL", "OFF")
        .define("HAVE_MYSQL", read_cmake_opt("build-with-mysql"))
        .define("HAVE_POSTGRESQL", read_cmake_opt("build-with-postgres"))
        .define("HAVE_PYTHON", "OFF")
        .define("HAVE_PYTHON3", "OFF")
        .define("QUICKFIX_SHARED_LIBS", "OFF")
        .define("QUICKFIX_EXAMPLES", "OFF")
        .define("QUICKFIX_TESTS", "OFF")
        // Always compile libquickfix in release mode.
        // We are not here to debug this library.
        .profile("RelWithDebInfo")
        .build();

    let quickfix_include_path = format!("{}/include", quickfix_dst.display());
    let quickfix_lib_path = format!("{}/lib", quickfix_dst.display());

    // Build quickfix C bind also as a static library.
    env::set_var("CMAKE_LIBRARY_PATH", [quickfix_lib_path].join(";"));

    let quickfix_bind_dst = Config::new(".")
        .cflag(format!("-I{quickfix_include_path}"))
        .cxxflag(format!("-I{quickfix_include_path}"))
        .define("QUICKFIX_BIND_EXAMPLES", "OFF")
        .define("HAVE_MYSQL", read_cmake_opt("build-with-mysql"))
        .define("HAVE_POSTGRESQL", read_cmake_opt("build-with-postgres"))
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

    // ⚠️ NOTE: libquickfix as a different name on windows with debug profile.
    println!("cargo:rustc-link-lib=static=quickfix");
    println!("cargo:rustc-link-lib=static=quickfixbind");

    // Lib std C++ is only available on UNIX platform.
    if let Some(lib_cpp_name) = target_os.lib_std_cpp_name() {
        println!("cargo:rustc-link-lib={lib_cpp_name}");
    }

    // Link with external libraries if needed.
    if have_feature("build-with-mysql") {
        println!("cargo:rustc-link-lib=mysqlclient");
    }
    if have_feature("build-with-postgres") {
        println!("cargo:rustc-link-lib=pq");
    }
}

fn update_sub_repositories() {
    if Path::new("libquickfix/LICENSE").exists() {
        return;
    }

    if !Command::new("git")
        .args(["submodule", "update", "--init", "--recursive"])
        .current_dir("..")
        .status()
        .expect("Fail to get command status")
        .success()
    {
        panic!("Fail to update sub repo");
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TargetOs {
    Windows,
    Linux,
    Other,
}

impl TargetOs {
    fn from_env() -> Self {
        match env::var("CARGO_CFG_TARGET_OS").as_deref() {
            Ok("windows") => Self::Windows,
            Ok("linux") => Self::Linux,
            _ => Self::Other,
        }
    }

    fn lib_std_cpp_name(&self) -> Option<&'static str> {
        match self {
            Self::Windows => None,
            Self::Linux => Some("stdc++"),
            Self::Other => Some("c++"),
        }
    }
}
