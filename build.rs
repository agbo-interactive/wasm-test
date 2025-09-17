use std::{path::PathBuf, process::Command};

fn main() {
    let project_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/guest-module"));
    let project_dir = project_dir.canonicalize().unwrap_or(project_dir);
    println!("cargo::rerun-if-changed={project_dir:?}");
    println!("Building WASM in {project_dir:?}");
    let output = Command::new("cargo")
        .args([
            "build",
            "--target",
            "wasm32-unknown-unknown",
            "--config",
            "profile.dev.opt-level=0",
            "--config",
            "profile.dev.debug=true",
        ])
        .current_dir(project_dir)
        .output()
        .expect("failed to execute process");
    println!("Build Output:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("Build Errs:\n{}", String::from_utf8_lossy(&output.stderr));
    if !output.status.success() {
        panic!("Build failed");
    }
}
