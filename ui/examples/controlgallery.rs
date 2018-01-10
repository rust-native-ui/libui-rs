//! An example control gallery: a port of the same `ui` example.

extern crate ui;

use std::rc::Rc;

use ui::{UI, BoxControl, Button, Checkbox, ColorButton, Combobox, EditableCombobox, DateTimePicker,
         Entry, FontButton, Group, Label, Menu, MenuItem, ProgressBar, RadioButtons, Separator, Slider, Spinbox, Tab, Window};

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
        mainwin.on_closing(Box::new(move |_| {
            ui.quit();
            false
        }));
    }

    let vbox = BoxControl::new_vertical();
    vbox.set_padded(true);
    mainwin.set_child(vbox.clone().into());

    let hbox = BoxControl::new_horizontal();
    hbox.set_padded(true);
    vbox.append(hbox.clone().into(), true);

    let group = Group::new("Basic Controls");
    group.set_margined(true);
    hbox.append(group.clone().into(), false);

    let inner = BoxControl::new_vertical();
    inner.set_padded(true);
    group.set_child(inner.clone().into());

    inner.append(Button::new("Button").into(), false);
    inner.append(Checkbox::new("Checkbox").into(), false);
    let entry = Entry::new();
    entry.set_text("Entry");
    inner.append(entry.into(), false);
    inner.append(Label::new("Label").into(), false);
    inner.append(Separator::new_horizontal().into(), false);

    inner.append(DateTimePicker::new_date_picker().into(), false);
    inner.append(DateTimePicker::new_time_picker().into(), false);
    inner.append(DateTimePicker::new_date_time_picker().into(), false);

    inner.append(FontButton::new().into(), false);
    inner.append(ColorButton::new().into(), false);

    let inner2 = BoxControl::new_vertical();
    inner2.set_padded(true);
    hbox.append(inner2.clone().into(), true);

    let group = Group::new("Numbers");
    group.set_margined(true);
    inner2.append(group.clone().into(), false);

    let inner = BoxControl::new_vertical();
    inner.set_padded(true);
    group.set_child(inner.clone().into());

    let spinbox = Spinbox::new(0, 100);
    spinbox.on_changed(Box::new(|spinbox| update(spinbox.value())));
    inner.append(spinbox.into(), false);

    let slider = Slider::new(0, 100);
    slider.on_changed(Box::new(|slider| update(slider.value())));
    inner.append(slider.into(), false);

    let progress_bar = ProgressBar::new();
    inner.append(progress_bar.into(), false);

    let group = Group::new("Lists");
    group.set_margined(true);
    inner2.append(group.clone().into(), false);

    let inner = BoxControl::new_vertical();
    inner.set_padded(true);
    group.set_child(inner.clone().into());

    let cbox = Combobox::new();
    cbox.append("Combobox Item 1");
    cbox.append("Combobox Item 2");
    cbox.append("Combobox Item 3");
    inner.append(cbox.into(), false);

    let cbox = EditableCombobox::new();
    cbox.append("Editable Item 1");
    cbox.append("Editable Item 2");
    cbox.append("Editable Item 3");
    inner.append(cbox.into(), false);


    let rb = RadioButtons::new();
    rb.append("Radio Button 1");
    rb.append("Radio Button 2");
    rb.append("Radio Button 3");
    inner.append(rb.into(), true);

    let tab = Tab::new();
    tab.append("Page 1", BoxControl::new_horizontal().into());
    tab.append("Page 2", BoxControl::new_horizontal().into());
    tab.append("Page 3", BoxControl::new_horizontal().into());
    inner2.append(tab.into(), true);

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
