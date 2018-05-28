extern crate cmake;
use cmake::Config;

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Fetch the submodule if needed
    if cfg!(feature = "fetch") {
        // Init or update the submodule with libui if needed
        if !Path::new("libui/.git").exists() {
            Command::new("git")
                .args(&["version"])
                .status()
                .expect("Git does not appear to be installed. Error");
            Command::new("git")
                .args(&["submodule", "update", "--init"])
                .status()
                .expect("Unable to init libui submodule. Error");
        } else {
            Command::new("git")
                .args(&["submodule", "update", "--recursive"])
                .status()
                .expect("Unable to update libui submodule. Error");
        }
    }

    // Deterimine if we're building for MSVC
    let target = env::var("TARGET").unwrap();
    let msvc = target.contains("msvc");
    // Build libui if needed. Otherwise, assume it's in lib/
    let mut dst;
    if cfg!(feature = "build") {
        dst = Config::new("libui").build_target("").profile("release").build();

        let mut postfix = Path::new("build").join("out");
        if msvc {
            postfix = postfix.join("Release");
        }
        dst = dst.join(&postfix);
    } else {
        dst = env::current_dir()
            .expect("Unable to retrieve current directory location.");
        dst.push("lib");
    }
    
    
    let libname;
     if msvc {
        libname = "libui";
    } else {
        libname = "ui";
    }

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib={}", libname);
}
