//! Demonstrates a mutable application state manipulated over a number of UIs

extern crate iui;
use iui::prelude::*;
use iui::controls::{Label, Spinbox, Slider, Entry, PasswordEntry, MultilineEntry, VerticalBox, HorizontalBox, HorizontalSeparator, Group, Spacer, ProgressBar};
use std::rc::Rc;
use std::cell::RefCell;

/// This struct will hold the values that multiple callbacks will need to access.
struct State {
    slider_val: i32,
    spinner_val: i32,
    entry_val: String,
    password_val: String,
    multi_val: String,
}

fn main() {
    // Initialize the UI framework.
    let ui = UI::init().unwrap();

    // Initialize the state of the application.
    let state = Rc::new(RefCell::new(State { slider_val: 0, spinner_val: 0, entry_val: "".into(), password_val: "".into(), multi_val: "".into() }));

    // Set up the inputs for the application.
    // While it's not necessary to create a block for this, it makes the code a lot easier
    // to read; the indentation presents a visual cue informing the reader that these
    // statements are related.
    let (input_group, mut slider, mut spinner, mut entry, mut password, mut multi) = {
        // The group will hold all the inputs
        let mut input_group = Group::new(&ui, "Inputs");
        // The vertical box arranges the inputs within the groups
        let mut input_vbox = VerticalBox::new(&ui);
        input_vbox.set_padded(&ui, true);
        // Numerical inputs
        let slider = Slider::new(&ui, 1, 100);
        let spinner = Spinbox::new(&ui, 1, 100);
        let entry = Entry::new(&ui);
        let password = PasswordEntry::new(&ui);
        let multi = MultilineEntry::new(&ui);
        // Add everything in hierarchy
        // Note the reverse order here. Again, it's not necessary, but it improves
        // readability.
        input_vbox.append(&ui, slider.clone(), LayoutStrategy::Compact);
        input_vbox.append(&ui, spinner.clone(), LayoutStrategy::Compact);
        input_vbox.append(&ui, Spacer::new(&ui), LayoutStrategy::Compact);
        input_vbox.append(&ui, HorizontalSeparator::new(&ui), LayoutStrategy::Compact);
        input_vbox.append(&ui, Spacer::new(&ui), LayoutStrategy::Compact);
        input_vbox.append(&ui, entry.clone(), LayoutStrategy::Compact);
        input_vbox.append(&ui, password.clone(), LayoutStrategy::Compact);
        input_vbox.append(&ui, multi.clone(), LayoutStrategy::Stretchy);
        input_group.set_child(&ui, input_vbox);
        (input_group, slider, spinner, entry, password, multi)
    };

    // Set up the outputs for the application. Organization is very similar to the
    // previous setup.
    let (output_group, add_label, sub_label, text_label, password_label, bigtext_label, progress_bar) = {
        let mut output_group = Group::new(&ui, "Outputs");
        let mut output_vbox = VerticalBox::new(&ui);
        let add_label = Label::new(&ui, "");
        let sub_label = Label::new(&ui, "");
        let text_label = Label::new(&ui, "");
        let password_label = Label::new(&ui, "");
        let bigtext_label = Label::new(&ui, "");
        let progress_bar = ProgressBar::indeterminate(&ui);
        output_vbox.append(&ui, add_label.clone(), LayoutStrategy::Compact);
        output_vbox.append(&ui, sub_label.clone(), LayoutStrategy::Compact);
        output_vbox.append(&ui, progress_bar.clone(), LayoutStrategy::Compact);
        output_vbox.append(&ui, text_label.clone(), LayoutStrategy::Compact);
        output_vbox.append(&ui, password_label.clone(), LayoutStrategy::Compact);
        output_vbox.append(&ui, bigtext_label.clone(), LayoutStrategy::Stretchy);
        output_group.set_child(&ui, output_vbox);
        (output_group, add_label, sub_label, text_label, password_label, bigtext_label, progress_bar)
    };

    // This horizontal box will arrange the two groups of controls.
    let mut hbox = HorizontalBox::new(&ui);
    hbox.append(&ui, input_group, LayoutStrategy::Stretchy);
    hbox.append(&ui, output_group, LayoutStrategy::Stretchy);

    // The window allows all constituent components to be displayed.
    let mut window = Window::new(&ui, "Input Output Test", 300, 150, WindowType::NoMenubar);
    window.set_child(&ui, hbox);
    window.show(&ui);

    // These on_changed functions allow updating the application state when a
    // control changes its value.

    slider.on_changed(&ui, {
        let state = state.clone();
        move |val| { state.borrow_mut().slider_val = val; }
    });

    spinner.on_changed(&ui, {
        let state = state.clone();
        move |val| { state.borrow_mut().spinner_val = val; }
    });

    entry.on_changed(&ui, {
        let state = state.clone();
        move |val| { state.borrow_mut().entry_val = val; }
    });

    password.on_changed(&ui, {
        let state = state.clone();
        move |val| { state.borrow_mut().password_val = val; }
    });

    multi.on_changed(&ui, {
        let state = state.clone();
        move |val| { state.borrow_mut().multi_val = val; }
    });


    // Rather than just invoking ui.run(), using EventLoop gives a lot more control
    // over the user interface event loop.
    // Here, the on_tick() callback is used to update the view against the state.
    let mut event_loop = ui.event_loop();
    event_loop.on_tick(&ui, {
        let ui = ui.clone();
        let mut add_label = add_label.clone();
        let mut sub_label = sub_label.clone();
        let mut text_label = text_label.clone();
        let mut password_label = password_label.clone();
        let mut bigtext_label = bigtext_label.clone();
        let mut progress_bar = progress_bar.clone();
        move || {
            let state = state.borrow();

            // Update all the outputs
            add_label.set_text(&ui, &format!("Added: {}", state.slider_val + state.spinner_val));
            sub_label.set_text(&ui, &format!("Subtracted: {}", state.slider_val - state.spinner_val));
            text_label.set_text(&ui, &format!("Text: {}", state.entry_val));
            password_label.set_text(&ui, &format!("Secret Text: {}", state.password_val));
            bigtext_label.set_text(&ui, &format!("Multiline Text: {}", state.multi_val));
            progress_bar.set_value(&ui, (state.slider_val + state.spinner_val) as u32)
        }
    });
    event_loop.run(&ui);
}
