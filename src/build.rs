use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("vineflower.jar");
    fs::copy("assets/vineflower.jar", &dest_path)
        .expect("failed to copy vineflower.jar");
}