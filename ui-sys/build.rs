extern crate make_cmd;

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    if !Path::new("libui/.git").exists() {
        Command::new("git").args(&["submodule", "update", "--init"]).status().unwrap();
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let outdir_argument = format!("OUTDIR={}", out_dir);
    let objdir_argument = format!("OBJDIR={}/obj", out_dir);
    make_cmd::gnu_make().args(&["-C", "libui", &*outdir_argument, &*objdir_argument])
                        .status()
                        .unwrap();

    println!("cargo:rustc-link-lib=dylib=ui");
    println!("cargo:rustc-link-search=native={}", out_dir);
}

