use ffi_utils;
use ui_sys;

/// The data of an image and its associated metadata.
pub struct Image {
    pub ui_image: *mut ui_sys::uiImage,
    pub data: Vec<u8>,
    pub width: f64,
    pub height: f64,
}

impl Image {
    pub fn new(x: f64, y: f64) -> Image {
        ffi_utils::ensure_initialized();
        unsafe {
            Image {
                ui_image: ui_sys::uiNewImage(x, y),
                data: Vec::with_capacity((x * y * 4.0) as usize),
                width: x,
                height: y,
            }
        }
    }
}
impl Drop for Image {
    fn drop(&mut self) {
        ffi_utils::ensure_initialized();
        unsafe {
            ui_sys::uiFreeImage(self.ui_image);
        }
    }
}
