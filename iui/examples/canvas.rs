extern crate iui;
extern crate ui_sys;

use iui::controls::{Area, AreaDrawParams, AreaHandler, HorizontalBox, LayoutStrategy};
use iui::draw::{Brush, FillMode, Path, SolidBrush};
use iui::prelude::*;
use std::f64::consts::PI;

struct HandleCanvas {}
impl AreaHandler for HandleCanvas {
    fn draw(&mut self, _area: &Area, draw_params: &AreaDrawParams) {
        let ctx = &draw_params.context;

        let path = Path::new(ctx, FillMode::Winding);
        path.add_rectangle(ctx, 0., 0., draw_params.area_width, draw_params.area_height);
        path.end(ctx);

        let brush = Brush::Solid(SolidBrush {
            r: 0.2,
            g: 0.6,
            b: 0.8,
            a: 1.,
        });

        draw_params.context.fill(&path, &brush);

        let path = Path::new(ctx, FillMode::Winding);
        for i in 0..100 {
            let x = i as f64 / 100.;
            let y = ((x * PI * 2.).sin() + 1.) / 2.;
            path.add_rectangle(
                ctx,
                x * draw_params.area_width,
                0.,
                draw_params.area_width / 100.,
                y * draw_params.area_height,
            );
        }
        path.end(ctx);

        let brush = Brush::Solid(SolidBrush {
            r: 0.2,
            g: 0.,
            b: 0.3,
            a: 1.,
        });

        draw_params.context.fill(&path, &brush);
    }
}

fn main() {
    let ui = UI::init().expect("Couldn't initialize UI library");
    let mut win = Window::new(&ui, "Area Canvas Example", 200, 200, WindowType::NoMenubar);

    let mut hbox = HorizontalBox::new(&ui);
    let area = Area::new(&ui, Box::new(HandleCanvas {}));
    hbox.append(&ui, area, LayoutStrategy::Stretchy);

    win.set_child(&ui, hbox);
    win.show(&ui);
    ui.main();
}
