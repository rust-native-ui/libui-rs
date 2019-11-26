//! Demonstrates a mutable application state manipulated over a number of UIs
//! Using the UIGrid function for a prettier interface.

extern crate iui;
use iui::prelude::*;
use iui::controls::{Label, Spinbox, Slider, PasswordEntry, Entry, MultilineEntry, LayoutGrid,
    GridAlignment, GridExpand, HorizontalSeparator, ProgressBar};
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

    // Create the grid which we'll use to lay out controls
    let mut grid = LayoutGrid::new(&ui);
    grid.set_padded(&ui, true);

    // Set up the inputs for the application.
    // While it's not necessary to create a block for this, it makes the code a lot easier
    // to read; the indentation presents a visual cue informing the reader that these
    // statements are related.
    let (mut slider, mut spinner, mut entry, mut password, mut multi) = {
        // Numerical inputs
        let slider = Slider::new(&ui, 1, 100);
        let spinner = Spinbox::new(&ui, 1, 100);
        // Text inputs
        let entry = Entry::new(&ui);
        let password = PasswordEntry::new(&ui);
        let multi = MultilineEntry::new(&ui);
        // Add everything into the grid
        grid.append(&ui, slider.clone(),
            // This is position (by slot) and size, expansion, and alignment.
            // In this case, row 0, col 0, 1 by 1, compress as much as possible,
            // and align to the fill.
            0, 0, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        grid.append(&ui, spinner.clone(),
            // This one is at column zero, row 1.
            0, 1, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        grid.append(&ui, HorizontalSeparator::new(&ui),
            0, 3, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        grid.append(&ui, entry.clone(),
            0, 4, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        grid.append(&ui, password.clone(),
            0, 5, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        grid.append(&ui, multi.clone(),
            // The multiline entry is at column 0, row 1, and expands vertically.
            0, 6, 1, 1, GridExpand::Vertical, GridAlignment::Fill, GridAlignment::Fill);
        (slider, spinner, entry, password, multi)
    };

    // Set up the outputs for the application. Organization is very similar to the
    // previous setup.
    let (add_label, sub_label, text_label, password_label, bigtext_label, progress_bar) = {
        let add_label = Label::new(&ui, "");
        let sub_label = Label::new(&ui, "");
        let text_label = Label::new(&ui, "");
        let password_label = Label::new(&ui, "");
        let bigtext_label = Label::new(&ui, "");
        let progress_bar = ProgressBar::indeterminate(&ui);
        grid.append(&ui, add_label.clone(),
            1, 0, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        grid.append(&ui, sub_label.clone(),
            1, 1, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        // We skip the #2 & 3 slots so that the text labels will align with their inputs.
        // This is important because the big text label can expand vertically.
        grid.append(&ui, text_label.clone(),
            1, 4, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        grid.append(&ui, password_label.clone(),
            1, 5, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        grid.append(&ui, bigtext_label.clone(),
            1, 6, 1, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        grid.append(&ui, progress_bar.clone(),
            0, 7, 2, 1, GridExpand::Neither, GridAlignment::Fill, GridAlignment::Fill);
        (add_label, sub_label, text_label, password_label, bigtext_label, progress_bar)
    };

    // The window allows all constituent components to be displayed.
    let mut window = Window::new(&ui, "Input Output Test", 300, 150, WindowType::NoMenubar);
    window.set_child(&ui, grid);
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
            progress_bar.set_value(&ui, (state.slider_val + state.spinner_val) as u32);
        }
    });
    event_loop.run(&ui);
}
