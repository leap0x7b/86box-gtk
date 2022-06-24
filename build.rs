use std::{
    fs::{self},
    process::Command,
};
use libadwaita::gio::compile_resources;

const UI_DIR: &str = "src/ui";
const OUT_DIR: &str = "target/assets";

fn main() {
    let ui_files: Vec<String> = fs::read_dir(UI_DIR)
        .expect("Failed to read UI dir")
        .filter(|entry| {
            entry
                .as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .ends_with(".blp")
        })
        .map(|entry| format!("{UI_DIR}/{}", entry.unwrap().file_name().to_str().unwrap()))
        .collect();

    let output = Command::new("blueprint-compiler")
        .arg("batch-compile")
        .arg(OUT_DIR)
        .arg(UI_DIR)
        .arg(ui_files.join(" "))
        .output()
        .expect("Failed to run blueprint-compiler");

    if !output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        panic!("Could not compile UI, blueprint-compiler exited with:\n{stdout}\n{stderr}")
    }

    compile_resources(
        OUT_DIR,
        "src/app.gresource.xml",
        "86box.gresource",
    );
}
