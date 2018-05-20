//! Pixmap example: Initializes and draws and empty 100x100 pixmap

extern crate ui;

use std::sync::{Arc, Mutex};

use ui::{BoxControl, Window, InitOptions};
use ui::{Image, Area, AreaHandler, AreaKeyEvent, AreaDrawParams};

struct ImageAreaHandler {
    data: Arc<Mutex<Vec<u32>>>
}

impl AreaHandler for ImageAreaHandler {
    fn draw(&mut self, _area: &Area, area_draw_params: &AreaDrawParams) {
        let img = Image::new(100, 100);
        img.load_pixmap(0, 0, 100, 100, &*self.data.lock().unwrap());
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

    let mut data = Arc::new(Mutex::new(vec![0xff123456;100*100]));

    let area = Area::new(Box::new(ImageAreaHandler { data: data.clone() }));
    vbox.append((&area).into(),false);

    ::std::thread::spawn(move || {
        let mut color = 0x0000ff;
        loop {
            color = color << 4 | (color & 0xff000000) >> 24;
            for j in 0..100 {
            ::std::thread::sleep_ms(10);
            {
                let mut d = data.lock().unwrap();
                for i in 0..100 {
                    d[i+j*100] = 0xff000000 | color;
                }
            }
            area.queue_redraw_all();
            }
        }
    });


    mainwin.show();
    ui::main();
}

pub fn main() {
    ui::init(InitOptions).unwrap();
    run();
    ui::uninit();
}
