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

    // On macOS, Python sometimes doesn't inherit system certs correctly.
    println!("cargo:rerun-if-changed=get-certificates.sh");

    let certs = Command::new("bash")
        .arg("get-certificates.sh")
        .status()
        .expect("Failed to run get-certificates.sh");

    if !certs.success() {
        panic!("Installing missing certs into Python environment failed");
    }
}
