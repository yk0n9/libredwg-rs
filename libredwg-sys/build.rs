use std::env::var;
use std::fs::copy;
use std::path::Path;

fn main() {
    println!("cargo:rustc-link-search={}", Path::new("libredwg-sys/lib").display());
    println!("cargo:rustc-link-lib=static=libredwg");

    let bin = Path::new("lib/libredwg.dll");
    let bin_name = bin.file_name().unwrap().to_str().unwrap();
    let out = var("OUT_DIR").unwrap();
    let target = Path::new(&out).join("../../..");
    if !target.join(bin_name).exists() {
        copy(bin, target.join(bin_name)).ok();
    }
}