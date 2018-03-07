extern crate cmake;
use cmake::Config;

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    if !Path::new("libui/.git").exists() {
        Command::new("git").args(&["submodule", "update", "--init"]).status().unwrap();
    }

    let target = env::var("TARGET").unwrap();
    let msvc = target.contains("msvc");

    let dst = Config::new("libui")
        .build_target("")
        .build();

    let mut postfix = Path::new("build").join("out");
    let libname;
    if msvc {
        postfix = postfix.join("Release");
        libname = "libui";
    } else {
        libname = "ui";
    }
    let dst = dst.join(&postfix);

    println!("cargo:rustc-link-lib={}", libname);
    println!("cargo:rustc-link-search=native={}", dst.display());
}
