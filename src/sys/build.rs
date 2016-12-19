extern crate cpp;

use std::process;

fn llvm_config(args: &[&str]) -> String {
    let output = process::Command::new("llvm-config").args(args).output().unwrap();
    assert!(output.status.success(), "llvm-config was unsuccessful");
    String::from_utf8(output.stdout).expect("invalid utf-8 from llvm-config")
}

fn find_llvm_include_dir() -> String { llvm_config(&["--includedir"]) }
fn find_llvm_lib_dir() -> String { llvm_config(&["--libdir"]) }

fn llvm_link_libraries() -> Vec<String> {
    llvm_config(&["--libs"]).split_whitespace()
        .map(|l| l.trim_left_matches("-l").to_owned()) // Remove the '-l' linker arg
        .collect()
}

fn main() {
    println!("cargo:rustc-link-search=native={}", find_llvm_lib_dir());

    for library in llvm_link_libraries() {
        println!("cargo:rustc-link-lib=static={}", library);
    }

    println!("cargo:rustc-link-lib=dylib={}", "z");

    cpp::build("lib.rs", "llvm-sys", |cfg| {
        cfg.flag("-std=c++11");
        cfg.include(find_llvm_include_dir());
    });
}
