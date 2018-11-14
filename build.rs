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

    run(Command::new("make")
        .arg(format!("OUT_DIR={}", out_dir))
        .current_dir(&Path::new("./src/cephes")));

    run(Command::new("make")
        .arg(format!("OUT_DIR={}", out_dir))
        .current_dir(&Path::new("./src/amos")));

    run(Command::new("make")
        .arg(format!("OUT_DIR={}", out_dir))
        .current_dir(&Path::new("./src/specfun")));

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=cdflib");
    println!("cargo:rustc-link-lib=static=md");
    println!("cargo:rustc-link-lib=static=amos");
    println!("cargo:rustc-link-lib=static=specfun");
    println!("cargo:rustc-link-lib=dylib=gfortran");
}
