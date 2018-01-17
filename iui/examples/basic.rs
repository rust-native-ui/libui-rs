extern crate iui;
use iui::prelude::*;
use iui::controls::{Label, Button, VerticalBox, Group};

fn main() {
    // Initialize the UI library
    let ui = UI::init().expect("Couldn't initialize UI library");
    // Create a window into which controls can be placed
    let win = Window::new(&ui, "Test App", 200, 200, WindowType::NoMenubar);
    
    // Create a vertical layout to hold the controls
    let vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);

    let group_vbox = VerticalBox::new(&ui);
    let group = Group::new(&ui, "Group");

    // Create two buttons to place in the window
    let button = Button::new(&ui, "Button");
    let quit_button = Button::new(&ui, "Quit");

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
