//! Functions and types related to 2D vector graphics.

use ffi::{self, uiDrawBrush, uiDrawBrushType, uiDrawContext, uiDrawPath};
use libc::c_int;
use std::marker::PhantomData;
use std::ptr;

pub use ffi::uiDrawBrushGradientStop as BrushGradientStop;
pub use ffi::uiDrawLineCap as LineCap;
pub use ffi::uiDrawLineJoin as LineJoin;
pub use ffi::uiDrawDefaultMiterLimit as DEFAULT_MITER_LIMIT;
pub use ffi::uiDrawFillMode as FillMode;
pub use ffi::uiDrawMatrix as Matrix;

pub struct Context {
    ui_draw_context: *mut uiDrawContext,
}

impl Context {
    #[inline]
    pub fn stroke(&self, path: &Path, brush: &Brush, stroke_params: &StrokeParams) {
        // TODO(pcwalton)
    }
}

#[derive(Clone, Debug)]
pub enum Brush {
    Solid(SolidBrush),
    LinearGradient(LinearGradientBrush),
    RadialGradient(RadialGradientBrush),
    Image,
}

#[derive(Clone, Debug)]
pub struct UiDrawBrushRef<'a> {
    draw_brush: uiDrawBrush,
    phantom: PhantomData<&'a uiDrawBrush>,
}

impl Brush {
    pub fn as_ui_draw_brush_ref(&self) -> UiDrawBrushRef {
        match *self {
            Brush::Solid(ref solid_brush) => {
                UiDrawBrushRef {
                    draw_brush: uiDrawBrush {
                        Type: uiDrawBrushType::Solid,

                        R: solid_brush.r,
                        G: solid_brush.g,
                        B: solid_brush.b,
                        A: solid_brush.a,

                        X0: 0.0,
                        Y0: 0.0,
                        X1: 0.0,
                        Y1: 0.0,
                        OuterRadius: 0.0,
                        Stops: ptr::null_mut(),
                        NumStops: 0,
                    },
                    phantom: PhantomData,
                }
            }
            Brush::LinearGradient(ref linear_gradient_brush) => {
                UiDrawBrushRef {
                    draw_brush: uiDrawBrush {
                        Type: uiDrawBrushType::LinearGradient,

                        R: 0.0,
                        G: 0.0,
                        B: 0.0,
                        A: 0.0,

                        X0: linear_gradient_brush.start_x,
                        Y0: linear_gradient_brush.start_y,
                        X1: linear_gradient_brush.end_x,
                        Y1: linear_gradient_brush.end_y,
                        OuterRadius: 0.0,
                        Stops: linear_gradient_brush.stops.as_ptr() as *mut BrushGradientStop,
                        NumStops: linear_gradient_brush.stops.len(),
                    },
                    phantom: PhantomData,
                }
            }
            Brush::RadialGradient(ref radial_gradient_brush) => {
                UiDrawBrushRef {
                    draw_brush: uiDrawBrush {
                        Type: uiDrawBrushType::RadialGradient,

                        R: 0.0,
                        G: 0.0,
                        B: 0.0,
                        A: 0.0,

                        X0: radial_gradient_brush.start_x,
                        Y0: radial_gradient_brush.start_y,
                        X1: radial_gradient_brush.outer_circle_center_x,
                        Y1: radial_gradient_brush.outer_circle_center_y,
                        OuterRadius: radial_gradient_brush.outer_radius,
                        Stops: radial_gradient_brush.stops.as_ptr() as *mut BrushGradientStop,
                        NumStops: radial_gradient_brush.stops.len(),
                    },
                    phantom: PhantomData,
                }
            }
            Brush::Image => {
                // These don't work yet in `libui`, but just for completeness' sake…
                UiDrawBrushRef {
                    draw_brush: uiDrawBrush {
                        Type: uiDrawBrushType::Image,

                        R: 0.0,
                        G: 0.0,
                        B: 0.0,
                        A: 0.0,

                        X0: 0.0,
                        Y0: 0.0,
                        X1: 0.0,
                        Y1: 0.0,
                        OuterRadius: 0.0,
                        Stops: ptr::null_mut(),
                        NumStops: 0,
                    },
                    phantom: PhantomData,
                }
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct SolidBrush {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[derive(Clone, Debug)]
pub struct LinearGradientBrush {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
    pub stops: Vec<BrushGradientStop>,
}

#[derive(Clone, Debug)]
pub struct RadialGradientBrush {
    pub start_x: f64,
    pub start_y: f64,
    pub outer_circle_center_x: f64,
    pub outer_circle_center_y: f64,
    pub outer_radius: f64,
    pub stops: Vec<BrushGradientStop>,
}

#[derive(Clone, Debug)]
pub struct StrokeParams {
    pub cap: LineCap,
    pub join: LineJoin,
    pub thickness: f64,
    pub miter_limit: f64,
    pub dashes: Vec<f64>,
    pub dash_phase: f64,
}

pub struct Path {
    ui_draw_path: *mut uiDrawPath,
}

impl Drop for Path {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::uiDrawFreePath(self.ui_draw_path)
        }
    }
}

impl Path {
    #[inline]
    pub fn new(fill_mode: FillMode) -> Path {
        unsafe {
            Path {
                ui_draw_path: ffi::uiDrawNewPath(fill_mode),
            }
        }
    }

    #[inline]
    pub fn new_figure(&self, x: f64, y: f64) {
        unsafe {
            ffi::uiDrawPathNewFigure(self.ui_draw_path, x, y)
        }
    }

    #[inline]
    pub fn new_figure_with_arc(&self,
                               x_center: f64,
                               y_center: f64,
                               radius: f64,
                               start_angle: f64,
                               sweep: f64,
                               negative: bool) {
        unsafe {
            ffi::uiDrawPathNewFigureWithArc(self.ui_draw_path,
                                            x_center,
                                            y_center,
                                            radius,
                                            start_angle,
                                            sweep,
                                            negative as c_int)
        }
    }

    #[inline]
    pub fn line_to(&self, x: f64, y: f64) {
        unsafe {
            ffi::uiDrawPathLineTo(self.ui_draw_path, x, y)
        }
    }

    #[inline]
    pub fn arc_to(&self,
                  x_center: f64,
                  y_center: f64,
                  radius: f64,
                  start_angle: f64,
                  sweep: f64,
                  negative: bool) {
        unsafe {
            ffi::uiDrawPathArcTo(self.ui_draw_path,
                                 x_center,
                                 y_center,
                                 radius,
                                 start_angle,
                                 sweep,
                                 negative as c_int)
        }
    }

    #[inline]
    pub fn bezier_to(&self, c1x: f64, c1y: f64, c2x: f64, c2y: f64, end_x: f64, end_y: f64) {
        unsafe {
            ffi::uiDrawPathBezierTo(self.ui_draw_path, c1x, c1y, c2x, c2y, end_x, end_y)
        }
    }

    #[inline]
    pub fn close_figure(&self) {
        unsafe {
            ffi::uiDrawPathCloseFigure(self.ui_draw_path)
        }
    }

    #[inline]
    pub fn add_rectangle(&self, x: f64, y: f64, width: f64, height: f64) {
        unsafe {
            ffi::uiDrawPathAddRectangle(self.ui_draw_path, x, y, width, height)
        }
    }

    #[inline]
    pub fn end(&self) {
        unsafe {
            ffi::uiDrawPathEnd(self.ui_draw_path)
        }
    }
}

