//! Demonstrates a mutable application state manipulated over a number of UIs

extern crate iui;
use iui::prelude::*;
use iui::controls::{Label, Spinbox, Slider, VerticalBox, HorizontalBox, Group};
use std::rc::Rc;
use std::cell::RefCell;

/// This struct will hold the values that multiple callbacks will need to access.
struct State {
    slider_val: i64,
    spinner_val: i64
}

fn main() {
    // Initialize the UI framework.
    let ui = UI::init().unwrap();

    // Initialize the state of the application.
    let state = Rc::new(RefCell::new(State { slider_val: 0, spinner_val: 0}));

    let input_group = Group::new(&ui, "Inputs");
    let input_vbox = VerticalBox::new(&ui);
    let slider = Slider::new(&ui, 1, 100);
    let spinner = Spinbox::new(&ui, 1, 100);
    input_vbox.append(&ui, slider.clone(), LayoutStrategy::Compact);
    input_vbox.append(&ui, spinner.clone(), LayoutStrategy::Compact);
    input_group.set_child(&ui, input_vbox);

    let output_group = Group::new(&ui, "Outputs");
    let output_vbox = VerticalBox::new(&ui);
    let add_label = Label::new(&ui, "");
    let sub_label = Label::new(&ui, "");
    output_vbox.append(&ui, add_label.clone(), LayoutStrategy::Compact);
    output_vbox.append(&ui, sub_label.clone(), LayoutStrategy::Compact);
    output_group.set_child(&ui, output_vbox);

    let hbox = HorizontalBox::new(&ui);
    hbox.append(&ui, input_group, LayoutStrategy::Stretchy);
    hbox.append(&ui, output_group, LayoutStrategy::Stretchy);

    let window = Window::new(&ui, "Input Output Test", 300, 150, WindowType::NoMenubar);
    window.set_child(&ui, hbox);
    window.show(&ui);

    let update_view = Rc::new({
        let ui = ui.clone();
        let add_label = add_label.clone();
        let sub_label = sub_label.clone();
        let state = state.clone();
        move || {
            let state = state.borrow();
            add_label.set_text(&ui, &format!("Added: {}", state.slider_val + state.spinner_val));
            sub_label.set_text(&ui, &format!("Subtracted: {}", state.slider_val - state.spinner_val));
        }
    });

    update_view();

    {
        let ui = ui.clone();
        let update_view = update_view.clone();
        let state = state.clone();
        slider.on_changed(&ui, |val| {
            state.borrow_mut().slider_val = val;
            update_view();
        });
    }

    {
        let ui = ui.clone();
        let update_view = update_view.clone();
        let state = state.clone();
        spinner.on_changed(&ui, |val| {
            state.borrow_mut().spinner_val = val;
            update_view();
        });
    }

    ui.main();
}
