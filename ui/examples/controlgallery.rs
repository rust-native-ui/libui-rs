//! An example control gallery: a port of the same `ui` example.

extern crate ui;

use ui::prelude::*;
use ui::controls::*;
use ui::menus::{Menu, MenuItem};

fn run(ui: Rc<UI>) {
    let menu = Menu::new("File");
    menu.append_item("Open").on_clicked(Box::new(open_clicked));
    menu.append_item("Save").on_clicked(Box::new(save_clicked));
    menu.append_separator();
    menu.append_quit_item();

    let menu = Menu::new("Edit");
    menu.append_check_item("Checkable Item");
    menu.append_separator();
    let item = menu.append_item("Disabled Item");
    item.disable();
    menu.append_preferences_item();

    let menu = Menu::new("Help");
    menu.append_item("Help");
    menu.append_about_item();

    let mainwin = Window::new("ui Control Gallery", 640, 480, true);
    mainwin.set_margined(true);
    {
        let ui = ui.clone();
        mainwin.on_closing(move |_| {
            ui.quit();
            false
        });
    }

    let vbox = BoxControl::new_vertical();
    vbox.set_padded(true);
    mainwin.set_child(vbox.deref().clone());

    let hbox = BoxControl::new_horizontal();
    hbox.set_padded(true);
    use std::ops::Deref;
    vbox.append(hbox.deref().clone());

    let group = Group::new("Basic Controls");
    group.set_margined(true);
    hbox.append(group.deref().clone());

    let inner = BoxControl::new_vertical();
    inner.set_padded(true);
    group.set_child(inner.deref().clone());

    inner.append(Button::new("Button"));
    inner.append(Checkbox::new("Checkbox"));
    let entry = Entry::new();
    entry.set_text("Entry");
    inner.append(entry);
    inner.append(Label::new("Label"));
    inner.append(Separator::new_horizontal());

    inner.append(DateTimePicker::new_date_picker());
    inner.append(DateTimePicker::new_time_picker());
    inner.append(DateTimePicker::new_date_time_picker());

    inner.append(FontButton::new());
    inner.append(ColorButton::new());

    let inner2 = BoxControl::new_vertical();
    inner2.set_padded(true);
    hbox.append(inner2.deref().clone());

    let group = Group::new("Numbers");
    group.set_margined(true);
    inner2.append(group.deref().clone());

    let inner = BoxControl::new_vertical();
    inner.set_padded(true);
    group.set_child(inner.deref().clone());

    let spinbox = Spinbox::new(0, 100);
    spinbox.on_changed(|spinbox| update(spinbox.value()));
    inner.append(spinbox);

    let slider = Slider::new(0, 100);
    slider.on_changed(|slider| update(slider.value()));
    inner.append(slider);

    let progress_bar = ProgressBar::new();
    inner.append(progress_bar);

    let group = Group::new("Lists");
    group.set_margined(true);
    inner2.append(group.deref().clone());

    let inner = BoxControl::new_vertical();
    inner.set_padded(true);
    group.set_child(inner.deref().clone());

    let cbox = Combobox::new();
    cbox.append("Combobox Item 1");
    cbox.append("Combobox Item 2");
    cbox.append("Combobox Item 3");
    inner.append(cbox);

    let cbox = EditableCombobox::new();
    cbox.append("Editable Item 1");
    cbox.append("Editable Item 2");
    cbox.append("Editable Item 3");
    inner.append(cbox);


    let rb = RadioButtons::new();
    rb.append("Radio Button 1");
    rb.append("Radio Button 2");
    rb.append("Radio Button 3");
    inner.append(rb);

    let tab = Tab::new();
    tab.append("Page 1", BoxControl::new_horizontal());
    tab.append("Page 2", BoxControl::new_horizontal());
    tab.append("Page 3", BoxControl::new_horizontal());
    inner2.append(tab);

    mainwin.show();
    ui.main();
}

pub fn main() {
    let ui = Rc::new(UI::init().unwrap());
    run(ui.clone());
}

fn open_clicked(_: &MenuItem, mainwin: &Window) {
    match mainwin.open_file() {
        Some(filename) => mainwin.msg_box("File selected", &*filename),
        None => mainwin.msg_box_error("No file selected", "Don't be alarmed!"),
    }
}

fn save_clicked(_: &MenuItem, mainwin: &Window) {
    match mainwin.open_file() {
        Some(filename) => {
            mainwin.msg_box(
                "File selected (don't worry, it's still there)",
                &*filename,
            )
        }
        None => mainwin.msg_box_error("No file selected", "Don't be alarmed!"),
    }
}

fn update(_: i64) {
    // TODO(pcwalton)
}
