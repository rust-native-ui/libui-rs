use ui::UI;
use libc::c_int;
use ui_sys::{self, uiDrawPath};

pub use ui_sys::uiDrawFillMode as FillMode;

pub struct Path {
    ui_draw_path: *mut uiDrawPath,
}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe { ui_sys::uiDrawFreePath(self.ui_draw_path) }
    }
}

impl Path {
    pub fn new(_ctx: &UI, fill_mode: FillMode) -> Path {
        unsafe {
            Path {
                ui_draw_path: ui_sys::uiDrawNewPath(fill_mode),
            }
        }
    }

    pub fn new_figure(&self, _ctx: &UI, x: f64, y: f64) {
        unsafe { ui_sys::uiDrawPathNewFigure(self.ui_draw_path, x, y) }
    }

    pub fn new_figure_with_arc(
        &self,
        _ctx: &UI,
        x_center: f64,
        y_center: f64,
        radius: f64,
        start_angle: f64,
        sweep: f64,
        negative: bool,
    ) {
        unsafe {
            ui_sys::uiDrawPathNewFigureWithArc(
                self.ui_draw_path,
                x_center,
                y_center,
                radius,
                start_angle,
                sweep,
                negative as c_int,
            )
        }
    }

    pub fn line_to(&self, _ctx: &UI, x: f64, y: f64) {
        unsafe { ui_sys::uiDrawPathLineTo(self.ui_draw_path, x, y) }
    }

    pub fn arc_to(
        &self,
        _ctx: &UI,
        x_center: f64,
        y_center: f64,
        radius: f64,
        start_angle: f64,
        sweep: f64,
        negative: bool,
    ) {
        unsafe {
            ui_sys::uiDrawPathArcTo(
                self.ui_draw_path,
                x_center,
                y_center,
                radius,
                start_angle,
                sweep,
                negative as c_int,
            )
        }
    }

    pub fn bezier_to(&self, _ctx: &UI, c1x: f64, c1y: f64, c2x: f64, c2y: f64, end_x: f64, end_y: f64) {
        unsafe { ui_sys::uiDrawPathBezierTo(self.ui_draw_path, c1x, c1y, c2x, c2y, end_x, end_y) }
    }

    pub fn close_figure(&self, _ctx: &UI) {
        unsafe { ui_sys::uiDrawPathCloseFigure(self.ui_draw_path) }
    }

    pub fn add_rectangle(&self, _ctx: &UI, x: f64, y: f64, width: f64, height: f64) {
        unsafe { ui_sys::uiDrawPathAddRectangle(self.ui_draw_path, x, y, width, height) }
    }

    pub fn end(&self, _ctx: &UI) {
        unsafe { ui_sys::uiDrawPathEnd(self.ui_draw_path) }
    }

    /// Return the underlying pointer for this Path.
    pub fn ptr(&self) -> *mut uiDrawPath {
        self.ui_draw_path
    }
}