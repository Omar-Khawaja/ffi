extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/c/calculate.c")
        .include("src")
        // generate static library
        .compile("libcalculate.a");
}
