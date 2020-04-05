extern crate bindgen;
extern crate cc;
extern crate embed_resource;
extern crate pkg_config;

use bindgen::Builder as BindgenBuilder;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // Deterimine build platform
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_triple = env::var("TARGET").unwrap();
    let msvc = target_triple.contains("msvc");
    let apple = target_triple.contains("apple");
    let unix = cfg!(target_family = "unix") && !apple;

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

    // Generate libui bindings on the fly
    let bindings = BindgenBuilder::default()
        .header("wrapper.h")
        .opaque_type("max_align_t") // For some reason this ends up too large
        //.rustified_enum(".*")
        .trust_clang_mangling(false) // clang sometimes wants to treat these functions as C++
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

    // Build libui if needed. Otherwise, assume it's in lib/
    if cfg!(feature = "build") {
        let mut base_config = cc::Build::new();
        let src_base = env::var("SRC_BASE").unwrap_or("libui".to_string());

        // Add source files that are common to all platforms
        base_config.include(format!("{}{}", src_base, "/common"));

        base_config.file(format!("{}{}", src_base, "/common/attribute.c"));
        base_config.file(format!("{}{}", src_base, "/common/attrlist.c"));
        base_config.file(format!("{}{}", src_base, "/common/attrstr.c"));
        base_config.file(format!("{}{}", src_base, "/common/areaevents.c"));
        base_config.file(format!("{}{}", src_base, "/common/control.c"));
        base_config.file(format!("{}{}", src_base, "/common/debug.c"));
        base_config.file(format!("{}{}", src_base, "/common/matrix.c"));
        base_config.file(format!("{}{}", src_base, "/common/opentype.c"));
        base_config.file(format!("{}{}", src_base, "/common/shouldquit.c"));
        base_config.file(format!("{}{}", src_base, "/common/tablemodel.c"));
        base_config.file(format!("{}{}", src_base, "/common/tablevalue.c"));
        base_config.file(format!("{}{}", src_base, "/common/userbugs.c"));
        base_config.file(format!("{}{}", src_base, "/common/utf.c"));

        if target_os == "windows" {
            base_config.cpp(true);
            base_config.include(format!("{}{}", src_base, "/windows"));

            base_config.file(format!("{}{}", src_base, "/windows/alloc.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/area.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/areadraw.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/areaevents.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/areascroll.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/areautil.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/attrstr.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/box.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/button.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/checkbox.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/colorbutton.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/colordialog.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/combobox.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/container.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/control.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/d2dscratch.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/datetimepicker.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/debug.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/draw.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/drawmatrix.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/drawpath.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/drawtext.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/dwrite.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/editablecombo.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/entry.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/events.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/fontbutton.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/fontdialog.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/fontmatch.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/form.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/graphemes.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/grid.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/group.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/image.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/init.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/label.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/main.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/menu.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/multilineentry.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/opentype.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/parent.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/progressbar.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/radiobuttons.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/separator.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/sizing.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/slider.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/spinbox.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/stddialogs.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/tab.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/table.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/tabledispinfo.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/tabledraw.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/tableediting.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/tablemetrics.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/tabpage.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/text.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/utf16.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/utilwin.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/window.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/winpublic.cpp"));
            base_config.file(format!("{}{}", src_base, "/windows/winutil.cpp"));

            // See https://github.com/nabijaczleweli/rust-embed-resource/issues/11
            let target = env::var("TARGET").unwrap();
            if let Some(tool) = cc::windows_registry::find_tool(target.as_str(), "cl.exe") {
                for (key, value) in tool.env() {
                    env::set_var(key, value);
                }
            }
            embed_resource::compile(format!("{}{}", src_base, "/windows/resources.rc"));

            link("user32", false);
            link("kernel32", false);
            link("gdi32", false);
            link("comctl32", false);
            link("uxtheme", false);
            link("msimg32", false);
            link("comdlg32", false);
            link("d2d1", false);
            link("dwrite", false);
            link("ole32", false);
            link("oleaut32", false);
            link("oleacc", false);
            link("uuid", false);
            link("windowscodecs", false);
        } else if unix {
            base_config.include(format!("{}{}", src_base, "/unix"));

            let pkg_cfg = pkg_config::Config::new().probe("gtk+-3.0").unwrap();
            for inc in pkg_cfg.include_paths {
                base_config.include(inc);
            }

            base_config.file(format!("{}{}", src_base, "/unix/alloc.c"));
            base_config.file(format!("{}{}", src_base, "/unix/area.c"));
            base_config.file(format!("{}{}", src_base, "/unix/attrstr.c"));
            base_config.file(format!("{}{}", src_base, "/unix/box.c"));
            base_config.file(format!("{}{}", src_base, "/unix/button.c"));
            base_config.file(format!("{}{}", src_base, "/unix/cellrendererbutton.c"));
            base_config.file(format!("{}{}", src_base, "/unix/checkbox.c"));
            base_config.file(format!("{}{}", src_base, "/unix/child.c"));
            base_config.file(format!("{}{}", src_base, "/unix/colorbutton.c"));
            base_config.file(format!("{}{}", src_base, "/unix/combobox.c"));
            base_config.file(format!("{}{}", src_base, "/unix/control.c"));
            base_config.file(format!("{}{}", src_base, "/unix/datetimepicker.c"));
            base_config.file(format!("{}{}", src_base, "/unix/debug.c"));
            base_config.file(format!("{}{}", src_base, "/unix/draw.c"));
            base_config.file(format!("{}{}", src_base, "/unix/drawmatrix.c"));
            base_config.file(format!("{}{}", src_base, "/unix/drawpath.c"));
            base_config.file(format!("{}{}", src_base, "/unix/drawtext.c"));
            base_config.file(format!("{}{}", src_base, "/unix/editablecombo.c"));
            base_config.file(format!("{}{}", src_base, "/unix/entry.c"));
            base_config.file(format!("{}{}", src_base, "/unix/fontbutton.c"));
            base_config.file(format!("{}{}", src_base, "/unix/fontmatch.c"));
            base_config.file(format!("{}{}", src_base, "/unix/form.c"));
            base_config.file(format!("{}{}", src_base, "/unix/future.c"));
            base_config.file(format!("{}{}", src_base, "/unix/graphemes.c"));
            base_config.file(format!("{}{}", src_base, "/unix/grid.c"));
            base_config.file(format!("{}{}", src_base, "/unix/group.c"));
            base_config.file(format!("{}{}", src_base, "/unix/image.c"));
            base_config.file(format!("{}{}", src_base, "/unix/label.c"));
            base_config.file(format!("{}{}", src_base, "/unix/main.c"));
            base_config.file(format!("{}{}", src_base, "/unix/menu.c"));
            base_config.file(format!("{}{}", src_base, "/unix/multilineentry.c"));
            base_config.file(format!("{}{}", src_base, "/unix/opentype.c"));
            base_config.file(format!("{}{}", src_base, "/unix/progressbar.c"));
            base_config.file(format!("{}{}", src_base, "/unix/radiobuttons.c"));
            base_config.file(format!("{}{}", src_base, "/unix/separator.c"));
            base_config.file(format!("{}{}", src_base, "/unix/slider.c"));
            base_config.file(format!("{}{}", src_base, "/unix/spinbox.c"));
            base_config.file(format!("{}{}", src_base, "/unix/stddialogs.c"));
            base_config.file(format!("{}{}", src_base, "/unix/tab.c"));
            base_config.file(format!("{}{}", src_base, "/unix/table.c"));
            base_config.file(format!("{}{}", src_base, "/unix/tablemodel.c"));
            base_config.file(format!("{}{}", src_base, "/unix/text.c"));
            base_config.file(format!("{}{}", src_base, "/unix/util.c"));
            base_config.file(format!("{}{}", src_base, "/unix/window.c"));
        } else if apple {
            base_config.include(format!("{}{}", src_base, "/darwin"));
            base_config.file(format!("{}{}", src_base, "/darwin/aat.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/alloc.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/area.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/areaevents.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/attrstr.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/autolayout.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/box.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/button.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/checkbox.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/colorbutton.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/combobox.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/control.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/datetimepicker.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/debug.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/draw.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/drawtext.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/editablecombo.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/entry.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/fontbutton.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/fontmatch.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/fonttraits.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/fontvariation.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/form.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/future.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/graphemes.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/grid.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/group.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/image.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/label.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/main.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/map.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/menu.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/multilineentry.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/opentype.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/progressbar.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/radiobuttons.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/scrollview.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/separator.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/slider.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/spinbox.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/stddialogs.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/tab.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/table.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/tablecolumn.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/text.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/undocumented.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/util.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/window.m"));
            base_config.file(format!("{}{}", src_base, "/darwin/winmoveresize.m"));
            println!("cargo:rustc-link-lib=framework=AppKit");
        } else {
            panic!("unrecognized platform! cannot build libui from source");
        }

        // Link everything together into `libui.a`.  This will get linked
        // together because of the `links="ui"` flag in the `Cargo.toml` file,
        // and because the `.compile()` function emits
        // `cargo:rustc-link-lib=static=ui`.
        base_config.compile("libui.a");
    } else {
        // If we're not building the library, then assume it's pre-built and
        // exists in `lib/`
        let mut dst = env::current_dir().expect("Unable to retrieve current directory location.");
        dst.push("lib");

        let libname = if msvc { "libui" } else { "ui" };

        println!("cargo:rustc-link-search=native={}", dst.display());
        println!("cargo:rustc-link-lib={}", libname);
    }
}

/// Tell cargo to link the given library, and optionally to bundle it in.
pub fn link(name: &str, bundled: bool) {
    let target = env::var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();
    if target.get(2) == Some(&"windows") {
        println!("cargo:rustc-link-lib=dylib={}", name);
        if bundled && target.get(3) == Some(&"gnu") {
            let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
            println!("cargo:rustc-link-search=native={}/{}", dir, target[0]);
        }
    } else {
        println!("cargo:rustc-link-lib=dylib={}", name);
    }
}

/// Add the given framework to the linker path
pub fn link_framework(name: &str) {
    println!("cargo:rustc-link-lib=framework={}", name);
}
