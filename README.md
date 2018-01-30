# The Improved User Interface Crate
[![libui-rs build status](https://api.travis-ci.org/LeoTindall/libui-rs.svg?branch=master)](https://travis-ci.org/LeoTindall/libui-rs/)

`iui` is a simple, small, easy to distribute GUI library, a Rusty user interface library that binds to platform native APIs.
These are work-in-progress bindings to the minimalistic native UI library [libui][libui].

Add this to your crate with:

```
iui = "0.1.0"
```

## Example

```
extern crate iui;
use iui::prelude::*;
use iui::controls::{VerticalBox, MultilineEntry, Button};
use std::io::prelude::*;
use std::error::Error;
use std::fs::File;

fn main() {
    // Initialize the UI
    let ui = UI::init().unwrap();

    // Create the input controls
    let entry = MultilineEntry::new(&ui);
    let button = Button::new(&ui, "Save Buffer");

    // Set up the application's layout
    let window = Window::new(&ui, "Save Buffer to File", 640, 480, WindowType::NoMenubar);
    let vbox = VerticalBox::new(&ui);
    vbox.append(&ui, entry.clone(), LayoutStrategy::Stretchy);
    vbox.append(&ui, button.clone(), LayoutStrategy::Compact);
    window.set_child(&ui, vbox);
    window.show(&ui);

    // When the button is clicked, get the name of a file and then write the entry's contents to it.
    // Note the in real code you should spin off a thread to do the actual writing, do it between UI events,
    // or use Tokio. Even with minmal content, this method shows noticable lag.
    button.on_clicked(&ui, {
        let ui = ui.clone();
        move |_| {
            if let Some(path) = window.save_file(&ui) {
                let mut file = match File::create(&path) {
                    Err(why) => { window.modal_err(&ui, "I/O Error", &format!("Could not open file {}: {}", path.display(), why.description())); return; }
                    Ok(f) => f
                };
                match file.write_all(entry.value(&ui).as_bytes()) {
                    Err(why) => { window.modal_err(&ui, "I/O Error", &format!("Could not write to file {}: {}", path.display(), why.description())); return; }
                    Ok(_) => ()
                };
            }    
        }
    });

    ui.main();
}
```

## Organization

`iui` is the safe Rust wrapper, to be used by most users.
`ui` is the old version of the safe wrapper. Don't use this.
`ui-sys` is the raw unsafe bindings to the `libui` C code. Requires `cmake` so it can build `libui`.

## Building
`libui` is included as a submodule. You will need CMake to build `libui` itself.

Based on work by @pcwalton. Licensed MIT.

## Testing Note
Travis does not connect video devices to their testing environments, so the tests cannot be run. Therefore, the "tests" only check compilation.

[libui]: https://github.com/andlabs/libui
