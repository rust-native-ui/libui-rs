//! Pixmap example: Initializes and draws and empty 100x100 pixmap

extern crate ui;

use ui::{BoxControl, Window, InitOptions};
use ui::{Image, Area, AreaHandler, AreaKeyEvent, AreaDrawParams};

struct ImageAreaHandler {
    data: Vec<u32>
}

impl AreaHandler for ImageAreaHandler {
    fn draw(&mut self, _area: &Area, area_draw_params: &AreaDrawParams) {
        let img = Image::new(100, 100);
        img.load_pixmap(0, 0, 100, 100, &self.data);
        area_draw_params.context.draw_image(0.0, 0.0, &img);
    }

    fn key_event(&mut self, _area: &Area, _area_key_event: &AreaKeyEvent) -> bool {
        println!("key");
        true
    }
}

fn run() {
    let mainwin = Window::new("ui Control Gallery", 640, 480, true);
    mainwin.set_margined(true);
    mainwin.set_autosave("libui-rs_image-example_main-window");
    mainwin.on_closing(Box::new(|_| {
        ui::quit();
        false
    }));

    let vbox = BoxControl::new_vertical();
    vbox.set_padded(true);
    mainwin.set_child(vbox.clone().into());

    let area = Area::new(Box::new(ImageAreaHandler {data: vec![0xff123456;100*100]}));
    vbox.append(area.into(),false);

    mainwin.show();
    ui::main();
}

pub fn main() {
    ui::init(InitOptions).unwrap();
    run();
    ui::uninit();
}
