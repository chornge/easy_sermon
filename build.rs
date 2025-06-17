use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=api/download.py");

    let model = Command::new("python3")
        .arg("api/download.py")
        .status()
        .expect("Failed to run download.py");

    if !model.success() {
        panic!("Model download failed");
    }
}
