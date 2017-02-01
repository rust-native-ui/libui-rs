use libc::c_void;
use ui_sys::{self, uiImage, uiImageData, uiNewImage,
             uiFreeImage, uiImageGetFormat, uiImageGetData,
             uiImageLoadPixmap32Raw, uiDrawImage};
use ui_sys::uiDrawContext;

pub struct Image {
    ui_image: *mut uiImage
}

// #define uiPixmap32FormatOffsets(a,r,g,b)    ((a) << 0 | (r) << 2 | (g) << 4 | (b) << 6)
const uiPixmap32FormatOffsetMask: u32        = 0x0ff;
const uiPixmap32FormatHasAlpha: u32          = 0x100;
const uiPixmap32FormatAlphaPremultiplied: u32    = 0x200;
const uiPixmap32FormatZeroRowBottom: u32         = 0x400;

impl Image {
    pub fn new(w: i32, h: i32) -> Image {
        unsafe {
            Image {
                ui_image: uiNewImage(w, h)
            }
        }
    }

    #[inline]
    pub fn as_ui_draw_image(&self) -> *const uiImage {
        self.ui_image
    }

    pub fn load_pixmap(&self, offset_x: i32, offset_y: i32, w: i32, h: i32, data: &[u32]) {
        unsafe {
            // uiImageLoadPixmap32Raw(uiImage *img, int x, int y, int width, int height,
            // int rowstrideBytes, uiPixmap32Format fmt, void *data);
            let img_data = get_image_data(self.ui_image);
            uiImageLoadPixmap32Raw(self.ui_image, offset_x, offset_y, w, h, w*4, img_data.fmt, data.as_ptr() as *const c_void);
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { uiFreeImage(self.ui_image) };
    }
}

fn get_image_data(img: *const uiImage) -> uiImageData {
    use std::ptr;
    let mut d = uiImageData {
        fmt: 0,
        width: 0,
        height: 0,
        rowstride: 0,
        data: ptr::null_mut(),
    };

    unsafe { uiImageGetData(img, &mut d) }

    d
}
