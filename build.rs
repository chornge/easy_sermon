use std::process::Command;

fn main() {
    // Optional: Rebuild only if api/download.py changes
    println!("cargo:rerun-if-changed=api/download.py");

    let status = Command::new("python3")
        .arg("api/download.py")
        .status()
        .expect("failed to run download.py");

    if !status.success() {
        panic!("Model download failed");
    }
}
