use std::env;

macro_rules! feature(($name:expr) => (env::var(concat!("CARGO_FEATURE_", $name)).is_ok()));

fn main() {
    if feature!("O3") {
        println!("cargo:rustc-link-lib=dylib=blas");
    } else {
    }
}
