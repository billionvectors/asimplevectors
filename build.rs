use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let lib_dir = manifest_dir.join("lib");

    let src = lib_dir.join("libatinyvectors.so");
    if !src.exists() {
        panic!("Source file {} does not exist", src.display());
    }

    let profile = env::var("PROFILE").unwrap();
    let target_dir = env::var("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("target"));
    let build_dir = target_dir.join(&profile);
    let dest = build_dir.join("libatinyvectors.so");

    fs::copy(&src, &dest).expect(&format!("Failed to copy {} to {}", src.display(), dest.display()));
    println!("cargo:rerun-if-changed={}", src.display());

    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-lib=dylib=atinyvectors");
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
}
