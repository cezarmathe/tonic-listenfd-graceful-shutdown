//! Build script for `tonic-graceful-shutdown`.

fn main() {
    println!("cargo:rerun-if-changed=api/");

    tonic_build::compile_protos("api/dummy.proto")
        .expect("Failed to compile proto.");
}
