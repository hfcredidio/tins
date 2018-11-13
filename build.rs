use std::env;
use std::path::Path;
use std::process::Command;

fn run(command: &mut Command) {
    println!("Running: {:?}", command);
    match command.status() {
        Ok(status) => if !status.success() {
            panic!("`{:?}` failed: {}", command, status);
        },
        Err(error) => {
            panic!("failed to execute `{:?}`: {}", command, error);
        }
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    run(Command::new("make")
        .arg(format!("OUT_DIR={}", out_dir))
        .current_dir(&Path::new("./src/cdflib")));

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=cdflib");
}
