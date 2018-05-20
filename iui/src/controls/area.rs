//! Provides a way to allocate an area in the window for custom drawing.

use controls::Control;
use ui::UI;
use draw;
use libc::c_int;
use std::mem;
use ui_sys::{self, uiArea, uiAreaDrawParams, uiAreaHandler, uiAreaKeyEvent, uiAreaMouseEvent,
             uiControl};
pub use ui_sys::uiExtKey as ExtKey;

pub trait AreaHandler {
    fn draw(&mut self, _area: &Area, _area_draw_params: &AreaDrawParams) {}
    fn mouse_event(&mut self, _area: &Area, _area_mouse_event: &AreaMouseEvent) {}
    fn mouse_crossed(&mut self, _area: &Area, _left: bool) {}
    fn drag_broken(&mut self, _area: &Area) {}
    fn key_event(&mut self, _area: &Area, _area_key_event: &AreaKeyEvent) -> bool {
        true
    }
}

#[repr(C)]
struct RustAreaHandler {
    ui_area_handler: uiAreaHandler,
    trait_object: Box<AreaHandler>,
}

impl RustAreaHandler {
    fn new(_ctx: &UI, trait_object: Box<AreaHandler>) -> Box<RustAreaHandler> {
        return Box::new(RustAreaHandler {
            ui_area_handler: uiAreaHandler {
                Draw: draw,
                MouseEvent: mouse_event,
                MouseCrossed: mouse_crossed,
                DragBroken: drag_broken,
                KeyEvent: key_event,
            },
            trait_object: trait_object,
        });

        extern "C" fn draw(
            ui_area_handler: *mut uiAreaHandler,
            ui_area: *mut uiArea,
            ui_area_draw_params: *mut uiAreaDrawParams,
        ) {
            unsafe {
                let area = Area::from_ui_area(ui_area);
                let area_draw_params =
                    AreaDrawParams::from_ui_area_draw_params(&*ui_area_draw_params);
                (*(ui_area_handler as *mut RustAreaHandler))
                    .trait_object
                    .draw(&area, &area_draw_params);
                mem::forget(area_draw_params);
                mem::forget(area);
            }
        }

        extern "C" fn mouse_event(
            ui_area_handler: *mut uiAreaHandler,
            ui_area: *mut uiArea,
            ui_area_mouse_event: *mut uiAreaMouseEvent,
        ) {
            unsafe {
                let area = Area::from_ui_area(ui_area);
                let area_mouse_event =
                    AreaMouseEvent::from_ui_area_mouse_event(&*ui_area_mouse_event);
                (*(ui_area_handler as *mut RustAreaHandler))
                    .trait_object
                    .mouse_event(&area, &area_mouse_event);
                mem::forget(area_mouse_event);
                mem::forget(area);
            }
        }

        extern "C" fn mouse_crossed(
            ui_area_handler: *mut uiAreaHandler,
            ui_area: *mut uiArea,
            left: c_int,
        ) {
            unsafe {
                let area = Area::from_ui_area(ui_area);
                (*(ui_area_handler as *mut RustAreaHandler))
                    .trait_object
                    .mouse_crossed(&area, left != 0);
                mem::forget(area);
            }
        }

        extern "C" fn drag_broken(ui_area_handler: *mut uiAreaHandler, ui_area: *mut uiArea) {
            unsafe {
                let area = Area::from_ui_area(ui_area);
                (*(ui_area_handler as *mut RustAreaHandler))
                    .trait_object
                    .drag_broken(&area);
                mem::forget(area);
            }
        }

        extern "C" fn key_event(
            ui_area_handler: *mut uiAreaHandler,
            ui_area: *mut uiArea,
            ui_area_key_event: *mut uiAreaKeyEvent,
        ) -> c_int {
            unsafe {
                let area = Area::from_ui_area(ui_area);
                let area_key_event = AreaKeyEvent::from_ui_area_key_event(&*ui_area_key_event);
                let result = (*(ui_area_handler as *mut RustAreaHandler))
                    .trait_object
                    .key_event(&area, &area_key_event);
                mem::forget(area_key_event);
                mem::forget(area);
                result as c_int
            }
        }
    }
}

define_control!{
    /// A control which takes up space on which the application can draw custom content.
    rust_type: Area,
    sys_type: uiArea
}

impl Area {
    pub fn new(ctx: &UI, area_handler: Box<AreaHandler>) -> Area {
        unsafe {
            let mut rust_area_handler = RustAreaHandler::new(ctx, area_handler);
            let area = Area::from_raw(ui_sys::uiNewArea(
                &mut *rust_area_handler as *mut RustAreaHandler as *mut uiAreaHandler,
            ));
            mem::forget(rust_area_handler);
            area
        }
    }

    pub fn new_scrolling(ctx: &UI, area_handler: Box<AreaHandler>, width: i64, height: i64) -> Area {
        unsafe {
            let mut rust_area_handler = RustAreaHandler::new(ctx, area_handler);
            let area = Area::from_raw(ui_sys::uiNewScrollingArea(
                &mut *rust_area_handler as *mut RustAreaHandler as *mut uiAreaHandler,
                width,
                height,
            ));
            mem::forget(rust_area_handler);
            area
        }
    }

    pub unsafe fn from_ui_area(ui_area: *mut uiArea) -> Area {
        Area { uiArea: ui_area }
    }

    pub fn set_size(&self, _ctx: &UI, width: i64, height: i64) {
        unsafe { ui_sys::uiAreaSetSize(self.uiArea, width, height) }
    }

    pub fn queue_redraw_all(&self, _ctx: &UI) {
        unsafe { ui_sys::uiAreaQueueRedrawAll(self.uiArea) }
    }

    pub fn scroll_to(&self, _ctx: &UI, x: f64, y: f64, width: f64, height: f64) {
        unsafe { ui_sys::uiAreaScrollTo(self.uiArea, x, y, width, height) }
    }
}

pub struct AreaDrawParams {
    pub context: draw::Context,

    pub area_width: f64,
    pub area_height: f64,

    pub clip_x: f64,
    pub clip_y: f64,
    pub clip_width: f64,
    pub clip_height: f64,
}

impl AreaDrawParams {
    // TODO: check if UI is initialized?
    unsafe fn from_ui_area_draw_params(ui_area_draw_params: &uiAreaDrawParams) -> AreaDrawParams {
        AreaDrawParams {
            context: draw::Context::from_ui_draw_context(ui_area_draw_params.Context),
            area_width: ui_area_draw_params.AreaWidth,
            area_height: ui_area_draw_params.AreaHeight,
            clip_x: ui_area_draw_params.ClipX,
            clip_y: ui_area_draw_params.ClipY,
            clip_width: ui_area_draw_params.ClipWidth,
            clip_height: ui_area_draw_params.ClipHeight,
        }
    }
}

bitflags! {
    pub flags Modifiers: u8 {
        const MODIFIER_CTRL = 1 << 0,
        const MODIFIER_ALT = 1 << 1,
        const MODIFIER_SHIFT = 1 << 2,
        const MODIFIER_SUPER = 1 << 3,
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AreaMouseEvent {
    pub x: f64,
    pub y: f64,

    pub area_width: f64,
    pub area_height: f64,

    pub down: i32,
    pub up: i32,

    pub count: i32,

    pub modifiers: Modifiers,

    pub held_1_to_64: u64,
}

impl AreaMouseEvent {
    // TODO: check if UI is initialized?
    pub fn from_ui_area_mouse_event(ui_area_mouse_event: &uiAreaMouseEvent) -> AreaMouseEvent {
        AreaMouseEvent {
            x: ui_area_mouse_event.X,
            y: ui_area_mouse_event.Y,
            area_width: ui_area_mouse_event.AreaWidth,
            area_height: ui_area_mouse_event.AreaHeight,
            down: ui_area_mouse_event.Down,
            up: ui_area_mouse_event.Up,
            count: ui_area_mouse_event.Count,
            modifiers: Modifiers::from_bits(ui_area_mouse_event.Modifiers as u8).unwrap(),
            held_1_to_64: ui_area_mouse_event.Held1To64,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AreaKeyEvent {
    pub key: u8,
    pub ext_key: ExtKey,
    pub modifier: Modifiers,
    pub modifiers: Modifiers,
    pub up: bool,
}

impl AreaKeyEvent {
    // TODO: check if UI is initialized?
    pub fn from_ui_area_key_event(ui_area_key_event: &uiAreaKeyEvent) -> AreaKeyEvent {
        AreaKeyEvent {
            key: ui_area_key_event.Key as u8,
            ext_key: ui_area_key_event.ExtKey,
            modifier: Modifiers::from_bits(ui_area_key_event.Modifier as u8).unwrap(),
            modifiers: Modifiers::from_bits(ui_area_key_event.Modifiers as u8).unwrap(),
            up: ui_area_key_event.Up != 0,
        }
    }
}
