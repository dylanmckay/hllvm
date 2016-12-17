extern crate gcc;

fn main() {
    gcc::Config::new()
        .cpp(true)
        .flag("-std=c++11")
        .file("lib.cpp")
        .file("IR/Value.cpp")
        .compile("libllvm-sys.a");
}
