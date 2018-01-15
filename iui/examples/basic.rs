extern crate iui;
use iui::prelude::*;
use iui::controls::{Button, VerticalBox};

fn main() {
    // Initialize the UI library
    let ui = UI::init().expect("Couldn't initialize UI library");
    // Create a window into which controls can be placed
    let win = Window::new(&ui, "Test App", 200, 200, WindowType::NoMenubar);
    
    // Create a vertical layout to hold the buttons
    let vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);

    let button = iui::controls::Button::new(&ui, "Button");
    let quit_button = iui::controls::Button::new(&ui, "Quit");

    vbox.append(&ui, button, LayoutStrategy::Compact);
    vbox.append(&ui, quit_button, LayoutStrategy::Compact);

    // Actually put the button in the window
    win.set_child(&ui, vbox);
    // Show the window
    win.show(&ui);
    // Run the application
    ui.main();
}
