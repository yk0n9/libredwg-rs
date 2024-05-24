use std::process::Command;

fn get_host() -> String {
    let rustc_verbose = Command::new("rustc")
        .args(["--version", "--verbose"])
        .output()
        .unwrap()
        .stdout;
    let rustc_verbose = String::from_utf8(rustc_verbose).unwrap_or_default();
    let v = rustc_verbose
        .trim()
        .split(if cfg!(windows) { "\r\n" } else { "\n" })
        .collect::<Vec<_>>();
    v.into_iter()
        .find(|s| s.starts_with("host"))
        .unwrap_or_default()
        .to_string()
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
