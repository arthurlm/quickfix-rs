use std::{env, path::PathBuf};

fn main() {
    let root_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_path = root_path.parent().expect("No target dir");
    let link_dir = target_path.join("target").join("quickfix-bind");

    println!("cargo:rustc-link-search={}", link_dir.as_path().display());
    println!("cargo:rustc-link-lib=quickfixbind");
}
