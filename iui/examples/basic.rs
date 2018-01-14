extern crate iui;
use iui::prelude::*;

fn main() {
        // Initialize the UI library
        let ui = UI::init().expect("Couldn't initialize UI library");
        // Create a window into which controls can be placed
        let win = Window::new(&ui, "Test App", 200, 200, WindowType::NoMenubar);
        // Create a button to place in the window
        let btn = iui::controls::Button::new(&ui, "Button");

        // Actually put the button in the window
        win.set_child(&ui, btn);
        // Show the window
        win.show(&ui);
        // Run the application
        ui.main();
}
