use std::mem;
use std::ops::Mul;
use ui_sys::{self, uiDrawMatrix};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Matrix {
    pub ui_matrix: uiDrawMatrix,
}

impl Matrix {
    pub fn from_ui_matrix(ui_matrix: &uiDrawMatrix) -> Matrix {
        Matrix {
            ui_matrix: *ui_matrix,
        }
    }

    pub fn identity() -> Matrix {
        unsafe {
            let mut matrix = mem::uninitialized();
            ui_sys::uiDrawMatrixSetIdentity(&mut matrix);
            Matrix::from_ui_matrix(&matrix)
        }
    }

    pub fn translate(&mut self, x: f64, y: f64) {
        unsafe { ui_sys::uiDrawMatrixTranslate(&mut self.ui_matrix, x, y) }
    }

    pub fn scale(&mut self, x_center: f64, y_center: f64, x: f64, y: f64) {
        unsafe { ui_sys::uiDrawMatrixScale(&mut self.ui_matrix, x_center, y_center, x, y) }
    }

    pub fn rotate(&mut self, x: f64, y: f64, angle: f64) {
        unsafe { ui_sys::uiDrawMatrixRotate(&mut self.ui_matrix, x, y, angle) }
    }

    pub fn skew(&mut self, x: f64, y: f64, xamount: f64, yamount: f64) {
        unsafe { ui_sys::uiDrawMatrixSkew(&mut self.ui_matrix, x, y, xamount, yamount) }
    }

    pub fn multiply(&mut self, src: &Matrix) {
        unsafe {
            ui_sys::uiDrawMatrixMultiply(
                &mut self.ui_matrix,
                &src.ui_matrix as *const uiDrawMatrix as *mut uiDrawMatrix,
            )
        }
    }

    pub fn invertible(&self) -> bool {
        unsafe {
            ui_sys::uiDrawMatrixInvertible(
                &self.ui_matrix as *const uiDrawMatrix as *mut uiDrawMatrix,
            ) != 0
        }
    }

    pub fn invert(&mut self) -> bool {
        unsafe { ui_sys::uiDrawMatrixInvert(&mut self.ui_matrix) != 0 }
    }

    pub fn transform_point(&self, mut point: (f64, f64)) -> (f64, f64) {
        unsafe {
            ui_sys::uiDrawMatrixTransformPoint(
                &self.ui_matrix as *const uiDrawMatrix as *mut uiDrawMatrix,
                &mut point.0,
                &mut point.1,
            );
            point
        }
    }

    pub fn transform_size(&self, mut size: (f64, f64)) -> (f64, f64) {
        unsafe {
            ui_sys::uiDrawMatrixTransformSize(
                &self.ui_matrix as *const uiDrawMatrix as *mut uiDrawMatrix,
                &mut size.0,
                &mut size.1,
            );
            size
        }
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(mut self, other: Matrix) -> Matrix {
        self.multiply(&other);
        self
    }
}