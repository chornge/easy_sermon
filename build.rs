use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=api/download_vosk.py");

    let model = Command::new("python3")
        .arg("api/download_vosk.py")
        .status()
        .expect("Failed to run api/download_vosk.py");

    if !model.success() {
        panic!("Vosk model download failed");
    }
}
