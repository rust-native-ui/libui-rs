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
        let src_path = |x| format!("{}/{}", src_base, x);

        // Add source files that are common to all platforms
        base_config.include(src_path("/common"));

        base_config.file(src_path("common/attribute.c"));
        base_config.file(src_path("common/attrlist.c"));
        base_config.file(src_path("common/attrstr.c"));
        base_config.file(src_path("common/areaevents.c"));
        base_config.file(src_path("common/control.c"));
        base_config.file(src_path("common/debug.c"));
        base_config.file(src_path("common/matrix.c"));
        base_config.file(src_path("common/opentype.c"));
        base_config.file(src_path("common/shouldquit.c"));
        base_config.file(src_path("common/tablemodel.c"));
        base_config.file(src_path("common/tablevalue.c"));
        base_config.file(src_path("common/userbugs.c"));
        base_config.file(src_path("common/utf.c"));

        if target_os == "windows" {
            base_config.cpp(true);
            base_config.include(src_path("/windows"));

            base_config.file(src_path("windows/alloc.cpp"));
            base_config.file(src_path("windows/area.cpp"));
            base_config.file(src_path("windows/areadraw.cpp"));
            base_config.file(src_path("windows/areaevents.cpp"));
            base_config.file(src_path("windows/areascroll.cpp"));
            base_config.file(src_path("windows/areautil.cpp"));
            base_config.file(src_path("windows/attrstr.cpp"));
            base_config.file(src_path("windows/box.cpp"));
            base_config.file(src_path("windows/button.cpp"));
            base_config.file(src_path("windows/checkbox.cpp"));
            base_config.file(src_path("windows/colorbutton.cpp"));
            base_config.file(src_path("windows/colordialog.cpp"));
            base_config.file(src_path("windows/combobox.cpp"));
            base_config.file(src_path("windows/container.cpp"));
            base_config.file(src_path("windows/control.cpp"));
            base_config.file(src_path("windows/d2dscratch.cpp"));
            base_config.file(src_path("windows/datetimepicker.cpp"));
            base_config.file(src_path("windows/debug.cpp"));
            base_config.file(src_path("windows/draw.cpp"));
            base_config.file(src_path("windows/drawmatrix.cpp"));
            base_config.file(src_path("windows/drawpath.cpp"));
            base_config.file(src_path("windows/drawtext.cpp"));
            base_config.file(src_path("windows/dwrite.cpp"));
            base_config.file(src_path("windows/editablecombo.cpp"));
            base_config.file(src_path("windows/entry.cpp"));
            base_config.file(src_path("windows/events.cpp"));
            base_config.file(src_path("windows/fontbutton.cpp"));
            base_config.file(src_path("windows/fontdialog.cpp"));
            base_config.file(src_path("windows/fontmatch.cpp"));
            base_config.file(src_path("windows/form.cpp"));
            base_config.file(src_path("windows/graphemes.cpp"));
            base_config.file(src_path("windows/grid.cpp"));
            base_config.file(src_path("windows/group.cpp"));
            base_config.file(src_path("windows/image.cpp"));
            base_config.file(src_path("windows/init.cpp"));
            base_config.file(src_path("windows/label.cpp"));
            base_config.file(src_path("windows/main.cpp"));
            base_config.file(src_path("windows/menu.cpp"));
            base_config.file(src_path("windows/multilineentry.cpp"));
            base_config.file(src_path("windows/opentype.cpp"));
            base_config.file(src_path("windows/parent.cpp"));
            base_config.file(src_path("windows/progressbar.cpp"));
            base_config.file(src_path("windows/radiobuttons.cpp"));
            base_config.file(src_path("windows/separator.cpp"));
            base_config.file(src_path("windows/sizing.cpp"));
            base_config.file(src_path("windows/slider.cpp"));
            base_config.file(src_path("windows/spinbox.cpp"));
            base_config.file(src_path("windows/stddialogs.cpp"));
            base_config.file(src_path("windows/tab.cpp"));
            base_config.file(src_path("windows/table.cpp"));
            base_config.file(src_path("windows/tabledispinfo.cpp"));
            base_config.file(src_path("windows/tabledraw.cpp"));
            base_config.file(src_path("windows/tableediting.cpp"));
            base_config.file(src_path("windows/tablemetrics.cpp"));
            base_config.file(src_path("windows/tabpage.cpp"));
            base_config.file(src_path("windows/text.cpp"));
            base_config.file(src_path("windows/utf16.cpp"));
            base_config.file(src_path("windows/utilwin.cpp"));
            base_config.file(src_path("windows/window.cpp"));
            base_config.file(src_path("windows/winpublic.cpp"));
            base_config.file(src_path("windows/winutil.cpp"));

            // See https://github.com/nabijaczleweli/rust-embed-resource/issues/11
            let target = env::var("TARGET").unwrap();
            if let Some(tool) = cc::windows_registry::find_tool(target.as_str(), "cl.exe") {
                for (key, value) in tool.env() {
                    env::set_var(key, value);
                }
            }
            embed_resource::compile(src_path("/windows/resources.rc"));

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
            base_config.include(src_path("/unix"));

            let pkg_cfg = pkg_config::Config::new().probe("gtk+-3.0").unwrap();
            for inc in pkg_cfg.include_paths {
                base_config.include(inc);
            }

            base_config.file(src_path("unix/alloc.c"));
            base_config.file(src_path("unix/area.c"));
            base_config.file(src_path("unix/attrstr.c"));
            base_config.file(src_path("unix/box.c"));
            base_config.file(src_path("unix/button.c"));
            base_config.file(src_path("unix/cellrendererbutton.c"));
            base_config.file(src_path("unix/checkbox.c"));
            base_config.file(src_path("unix/child.c"));
            base_config.file(src_path("unix/colorbutton.c"));
            base_config.file(src_path("unix/combobox.c"));
            base_config.file(src_path("unix/control.c"));
            base_config.file(src_path("unix/datetimepicker.c"));
            base_config.file(src_path("unix/debug.c"));
            base_config.file(src_path("unix/draw.c"));
            base_config.file(src_path("unix/drawmatrix.c"));
            base_config.file(src_path("unix/drawpath.c"));
            base_config.file(src_path("unix/drawtext.c"));
            base_config.file(src_path("unix/editablecombo.c"));
            base_config.file(src_path("unix/entry.c"));
            base_config.file(src_path("unix/fontbutton.c"));
            base_config.file(src_path("unix/fontmatch.c"));
            base_config.file(src_path("unix/form.c"));
            base_config.file(src_path("unix/future.c"));
            base_config.file(src_path("unix/graphemes.c"));
            base_config.file(src_path("unix/grid.c"));
            base_config.file(src_path("unix/group.c"));
            base_config.file(src_path("unix/image.c"));
            base_config.file(src_path("unix/label.c"));
            base_config.file(src_path("unix/main.c"));
            base_config.file(src_path("unix/menu.c"));
            base_config.file(src_path("unix/multilineentry.c"));
            base_config.file(src_path("unix/opentype.c"));
            base_config.file(src_path("unix/progressbar.c"));
            base_config.file(src_path("unix/radiobuttons.c"));
            base_config.file(src_path("unix/separator.c"));
            base_config.file(src_path("unix/slider.c"));
            base_config.file(src_path("unix/spinbox.c"));
            base_config.file(src_path("unix/stddialogs.c"));
            base_config.file(src_path("unix/tab.c"));
            base_config.file(src_path("unix/table.c"));
            base_config.file(src_path("unix/tablemodel.c"));
            base_config.file(src_path("unix/text.c"));
            base_config.file(src_path("unix/util.c"));
            base_config.file(src_path("unix/window.c"));
        } else if apple {
            base_config.include(src_path("/darwin"));
            base_config.file(src_path("darwin/aat.m"));
            base_config.file(src_path("darwin/alloc.m"));
            base_config.file(src_path("darwin/area.m"));
            base_config.file(src_path("darwin/areaevents.m"));
            base_config.file(src_path("darwin/attrstr.m"));
            base_config.file(src_path("darwin/autolayout.m"));
            base_config.file(src_path("darwin/box.m"));
            base_config.file(src_path("darwin/button.m"));
            base_config.file(src_path("darwin/checkbox.m"));
            base_config.file(src_path("darwin/colorbutton.m"));
            base_config.file(src_path("darwin/combobox.m"));
            base_config.file(src_path("darwin/control.m"));
            base_config.file(src_path("darwin/datetimepicker.m"));
            base_config.file(src_path("darwin/debug.m"));
            base_config.file(src_path("darwin/draw.m"));
            base_config.file(src_path("darwin/drawtext.m"));
            base_config.file(src_path("darwin/editablecombo.m"));
            base_config.file(src_path("darwin/entry.m"));
            base_config.file(src_path("darwin/fontbutton.m"));
            base_config.file(src_path("darwin/fontmatch.m"));
            base_config.file(src_path("darwin/fonttraits.m"));
            base_config.file(src_path("darwin/fontvariation.m"));
            base_config.file(src_path("darwin/form.m"));
            base_config.file(src_path("darwin/future.m"));
            base_config.file(src_path("darwin/graphemes.m"));
            base_config.file(src_path("darwin/grid.m"));
            base_config.file(src_path("darwin/group.m"));
            base_config.file(src_path("darwin/image.m"));
            base_config.file(src_path("darwin/label.m"));
            base_config.file(src_path("darwin/main.m"));
            base_config.file(src_path("darwin/map.m"));
            base_config.file(src_path("darwin/menu.m"));
            base_config.file(src_path("darwin/multilineentry.m"));
            base_config.file(src_path("darwin/opentype.m"));
            base_config.file(src_path("darwin/progressbar.m"));
            base_config.file(src_path("darwin/radiobuttons.m"));
            base_config.file(src_path("darwin/scrollview.m"));
            base_config.file(src_path("darwin/separator.m"));
            base_config.file(src_path("darwin/slider.m"));
            base_config.file(src_path("darwin/spinbox.m"));
            base_config.file(src_path("darwin/stddialogs.m"));
            base_config.file(src_path("darwin/tab.m"));
            base_config.file(src_path("darwin/table.m"));
            base_config.file(src_path("darwin/tablecolumn.m"));
            base_config.file(src_path("darwin/text.m"));
            base_config.file(src_path("darwin/undocumented.m"));
            base_config.file(src_path("darwin/util.m"));
            base_config.file(src_path("darwin/window.m"));
            base_config.file(src_path("darwin/winmoveresize.m"));
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
