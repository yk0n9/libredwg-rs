fn main() {
    let current = std::env::current_dir().unwrap();
    if target_triple::HOST.contains("msvc") {
        println!(
            "cargo:rustc-link-search={}",
            current.join("lib").join("msvc").display()
        );
        println!("cargo:rustc-link-lib=static=libredwg");
    } else {
        println!(
            "cargo:rustc-link-search={}",
            current.join("lib").join("gnu").display()
        );
        println!("cargo:rustc-link-lib=static=redwg");
    };
}
