use ui::UI;
use draw::{Path, Brush, StrokeParams, Matrix};
use ui_sys::{self, uiDrawContext, uiDrawMatrix};

use image;

/// Drawing context, used to 
pub struct DrawContext {
    ui_draw_context: *mut uiDrawContext,
}

impl DrawContext {
    /// Create a Context from a ui_draw_context pointer. 
    /// 
    /// # Unsafety
    /// If the pointer is invalid, this is memory-unsafe.
    /// If libui is not initialized, behavior will be inconsistent.
    pub unsafe fn from_ui_draw_context(ui_draw_context: *mut uiDrawContext) -> DrawContext {
        DrawContext {
            ui_draw_context: ui_draw_context,
        }
    }

    /// Draw a stroke on this DrawContext which runs along the given Path, with the given Brush and StrokeParams.
    pub fn stroke(&self, ctx: &UI, path: &Path, brush: &Brush, stroke_params: &StrokeParams) {
        unsafe {
            let brush = brush.as_ui_draw_brush_ref(ctx);
            let stroke_params = stroke_params.as_stroke_params_ref(ctx);
            ui_sys::uiDrawStroke(
                self.ui_draw_context,
                path.ptr(),
                brush.ptr(),
                stroke_params.ptr(),
            )
        }
    }

    /// Draw a fill on this DrawContext using the given Path using the given Brush.
    pub fn fill(&self, ctx: &UI, path: &Path, brush: &Brush) {
        unsafe {
            let brush = brush.as_ui_draw_brush_ref(ctx);
            ui_sys::uiDrawFill(
                self.ui_draw_context,
                path.ptr(),
                brush.ptr(),
            )
        }
    }

    /// Transform this DrawContext by the given Matrix.
    pub fn transform(&self, _ctx: &UI, matrix: &Matrix) {
        unsafe {
            ui_sys::uiDrawTransform(
                self.ui_draw_context,
                &matrix.ui_matrix as *const uiDrawMatrix as *mut uiDrawMatrix,
            )
        }
    }

    /// Open a modal allowing the user to save the contents of this DrawContext.
    pub fn save(&self, _ctx: &UI) {
        unsafe { ui_sys::uiDrawSave(self.ui_draw_context) }
    }

    /// Open a modal allowing the user to load the contents of a DrawContext onto this one.
    pub fn restore(&self, _ctx: &UI) {
        unsafe { ui_sys::uiDrawRestore(self.ui_draw_context) }
    }

    /// Draw the pixels from the given Image onto this DrawContext at the given position.
    pub fn draw_image(&self, x: f64, y: f64, img: &image::Image) {
        unsafe {
            ui_sys::uiDrawPixmapImage(self.ui_draw_context, x, y, img.as_ui_draw_image())
        }
    }

    pub fn scale_image(&self, x_scale: f64, y_scale: f64) {
        unsafe {
            ui_sys::uiScalePixmapImage(self.ui_draw_context, x_scale, y_scale)
        }
    }
}