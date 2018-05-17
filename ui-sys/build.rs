extern crate cmake;
use cmake::Config;

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Update the submodule with libui if needed
    if !Path::new("libui/.git").exists() {
        Command::new("git").args(&["version"]).status().expect("Git does not appear to be installed. Git is required to build ui-sys; install Git or build ui-sys independently. Error");
        Command::new("git").args(&["submodule", "update", "--init"]).status().expect("Unable to update Git submodules. Error");
    } else {
        Command::new("git").args(&["submodule", "update", "--recursive"]).status().expect("Unable to update Git submodules. Error");
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
