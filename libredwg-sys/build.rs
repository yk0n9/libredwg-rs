fn main() {
    let current = std::env::current_dir().unwrap();
    println!("cargo:rustc-link-search={}", current.join("lib").display());
    println!("cargo:rustc-link-lib=static=libredwg");
}