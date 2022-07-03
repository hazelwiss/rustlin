fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=lscripts/*");
    println!("cargo:rerun-if-changed=targets/*");
}
