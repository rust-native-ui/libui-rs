extern crate make_cmd;
extern crate cmake;

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Update the submodule with libui if needed
    if !Path::new("libui/.git").exists() {
        Command::new("git").args(&["submodule", "update", "--init"]).status().unwrap();
    }

    // Run cmake to build the project's makefiles
    let dst = cmake::Config::new("libui")
                             .build_target("")
                             .build();
    let dst = format!("{}/build", dst.display());

    // Run make to build the actual library
    let out_dir = env::var("OUT_DIR").unwrap();
    let outdir_argument = format!("OUTDIR={}", out_dir);
    let objdir_argument = format!("OBJDIR={}/obj", out_dir);
    make_cmd::gnu_make().args(&["-C", &dst, &*outdir_argument, &*objdir_argument])
                        .status()
                        .unwrap();
    //Command::new("cp").args(&["-r", "libui/out/", &*out_dir]).status().unwrap();

    println!("cargo:rustc-link-search=native={}/out/", dst);
    println!("cargo:rustc-link-lib=dylib=ui");
}

