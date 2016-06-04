extern crate make_cmd;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs::{self, File};
use std::io::Write;

fn main() {
    if !Path::new("libui/.git").exists() {
        Command::new("git").args(&["submodule", "update", "--init"]).status().expect("Could not update libui submodule");
    }
    
    let cwd = env::current_dir().unwrap();
    
    let libui_path = cwd.join("libui");
    env::set_current_dir(&libui_path).expect("Could not change dir");
    
    // Run CMake
    //let cmake_cache_path = (&libui_path).join("CMakeCache.txt");
    //if cmake_cache_path.exists() {
    //    fs::remove_file(cmake_cache_path).expect("Could not remove cmake cache");
    //}
    Command::new("cmake").arg(".").output().expect("cmake failed");
    
    // Run Make
    let libui_out_path = (&libui_path).join("out");
    if ! libui_out_path.exists() {
        println!("Creating out dir");
        fs::create_dir(&libui_out_path).expect("Could not create out dir");
    }
    make_cmd::gnu_make().status().expect("Make failed");
    
    // Copy the output to the build folder
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);
    for entry in fs::read_dir(&libui_out_path).unwrap() {
        let entry = entry.expect("IO error while reading");
        let name = entry.file_name();
        let target = out_path.join(name);
        fs::copy(&entry.path(), &target).expect("Could not copy file");
        println!("Copying file from {} to {}", entry.path().display(), target.display());
    }
    

    // Configure cargo
    println!("cargo:rustc-link-lib=dylib=ui");
    println!("cargo:rustc-link-search={}", out_dir);
    //panic!();
}

