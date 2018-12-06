use super::Control;
use std::mem;
use ui::UI;
use ui_sys::{self, uiControl, uiProgressBar};

/// An enum representing the value of a `ProgressBar`
pub enum ProgressBarValue {
    /// Represents a set, consistent percentage of the bar to be filled
    ///
    /// The value should be in the range 0..=100
    Determinate(u32),
    /// Represents an indeterminate value of the progress bar, useful
    /// if you don't know how much of the task being represented is completed
    Indeterminate,
}

define_control! {
  /// A bar that fills up with a set percentage, used to show completion of a
  ///
  /// # Values
  /// A `ProgressBar` can be either determinate or indeterminate. See [`ProgressBarValue`]
  /// for an explanation of the differences.
  ///
  /// [`ProgressBarValue`]: enum.ProgressBarValue.html
  rust_type: ProgressBar,
  sys_type: uiProgressBar,
}

impl ProgressBar {
    /// Create a new progress bar with a value of 0
    pub fn new() -> ProgressBar {
        unsafe { ProgressBar::from_raw(ui_sys::uiNewProgressBar()) }
    }

    /// Create a new indeterminate progress bar
    pub fn indeterminate() -> ProgressBar {
        let mut pb = ProgressBar::new();
        pb.set_indeterminate();
        pb
    }

    /// Set the value of the progress bar to a determinate value
    pub fn set_determinate(&mut self, value: u32) {
        self.set_value(ProgressBarValue::Determinate(value));
    }

    /// Set the value of the progress bar to be indeterminate
    pub fn set_indeterminate(&mut self) {
        self.set_value(ProgressBarValue::Indeterminate);
    }

    /// Set the value of the progress bar
    pub fn set_value(&mut self, value: ProgressBarValue) {
        let sys_value = match value {
            ProgressBarValue::Determinate(value) => {
                let value = if value > 100 { 100 } else { value };
                value as i32
            }
            ProgressBarValue::Indeterminate => -1,
        };
        unsafe { ui_sys::uiProgressBarSetValue(self.uiProgressBar, sys_value) }
    }

    /// Get the value of the progress bar
    pub fn value(&self) -> ProgressBarValue {
        let sys_value = unsafe { ui_sys::uiProgressBarValue(self.uiProgressBar) };
        if sys_value.is_negative() {
            assert!(
                sys_value == -1,
                "if ProgressBar value is negative it can only be -1"
            );
            ProgressBarValue::Indeterminate
        } else {
            ProgressBarValue::Determinate(sys_value as u32)
        }
    }
}
