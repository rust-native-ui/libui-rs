# Improved User Interface
## A cross-platform UI toolkit for Rust based on libui
[![libui-rs travis build status](https://api.travis-ci.org/LeoTindall/libui-rs.svg?branch=master)](https://travis-ci.org/LeoTindall/libui-rs/)
[![libui-rs appveyor build status badge](https://ci.appveyor.com/api/projects/status/github/leotindall/libui-rs)](https://ci.appveyor.com/project/LeoTindall/libui-rs)
![actively developed badge](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

iui: [![iui crates.io version badge](https://img.shields.io/crates/v/iui.svg)](https://crates.io/crates/iui/)
[![docs.rs for iui](https://docs.rs/iui/badge.svg)](https://docs.rs/iui)
ui-sys: [![ui-sys crates.io version badge](https://img.shields.io/crates/v/ui-sys.svg)](https://crates.io/crates/ui-sys/)
[![docs.rs for ui-sys](https://docs.rs/ui-sys/badge.svg)](https://docs.rs/ui)

`iui` is a simple, small, easy to distribute GUI library, a Rusty user interface library that binds to platform native APIs.
These are work-in-progress bindings to the minimalistic native UI library [libui][libui] via the `ui-sys` bindings crate.

Add `iui` to your project with:

```toml
iui = "0.3"
```

## Organization

This repository contains multiple Rust crates. Also be sure to look at our [changelog](CHANGELOG.md) and learn [how to contribute](CONTRIBUTING.md).

* `iui` is the safe Rust wrapper, to be used by most users.
* `ui-sys` is the raw unsafe bindings to the `libui` C code. Requires `cmake` so it can build `libui`.
* `libui` is included as a submodule. 

Based on work by [@pcwalton](https://github.com/pcwalton/). Licensed MIT.

## Example

![Three example GUI applications running on Linux](themed.png)

```rust
extern crate iui;
use iui::prelude::*;
use iui::controls::{Label, Button, VerticalBox, Group};

fn main() {
    // Initialize the UI library
    let ui = UI::init().expect("Couldn't initialize UI library");
    // Create a window into which controls can be placed
    let mut win = Window::new(&ui, "Test App", 200, 200, WindowType::NoMenubar);

    // Create a vertical layout to hold the controls
    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);

    let mut group_vbox = VerticalBox::new(&ui);
    let mut group = Group::new(&ui, "Group");

    // Create two buttons to place in the window
    let mut button = Button::new(&ui, "Button");
    button.on_clicked(&ui, {
        let ui = ui.clone();
        move |btn| {
            btn.set_text(&ui, "Clicked!");
        }
    });

    let mut quit_button = Button::new(&ui, "Quit");
    quit_button.on_clicked(&ui, {
        let ui = ui.clone();
        move |_| {
            ui.quit();
        }
    });

    // Create a new label. Note that labels don't auto-wrap!
    let mut label_text = String::new();
    label_text.push_str("There is a ton of text in this label.\n");
    label_text.push_str("Pretty much every unicode character is supported.\n");
    label_text.push_str("🎉 用户界面 사용자 인터페이스");
    let label = Label::new(&ui, &label_text);

    vbox.append(&ui, label, LayoutStrategy::Stretchy);
    group_vbox.append(&ui, button, LayoutStrategy::Compact);
    group_vbox.append(&ui, quit_button, LayoutStrategy::Compact);
    group.set_child(&ui, group_vbox);
    vbox.append(&ui, group, LayoutStrategy::Compact);

    // Actually put the button in the window
    win.set_child(&ui, vbox);
    // Show the window
    win.show(&ui);
    // Run the application
    ui.main();
}
```

## Building ui-sys

`ui-sys` includes `libui` as a sub-module and allows it to be built on-the-fly with the
default features `fetch` and `build. With `fetch disabled, it will simply build the
existing sources without updating them, and with `build` disabled it will build nothing,
assuming either a system or local (in `./lib/`) version of `libui` is available.

Note that _most of the time_, building `libui` on the fly is what you want. It does however
require a copy of cmake, essential build tools, et cetera.
