fn main() {
    let target = std::env::var("TARGET").unwrap();
    if !target.contains("wasm32") {
        println!("cargo:rustc-link-search=framework=/Library/Frameworks");
        println!("cargo:rustc-link-lib=SDL2");
    }
}