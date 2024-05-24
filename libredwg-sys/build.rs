use std::process::Command;

fn get_host() -> String {
    let host = Command::new("rustc")
        .args(["--version", "--verbose"])
        .output()
        .unwrap()
        .stdout;
    String::from_utf8(host).unwrap_or_default()
}

fn main() {
    let current = std::env::current_dir().unwrap();
    if get_host().contains("msvc") {
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
