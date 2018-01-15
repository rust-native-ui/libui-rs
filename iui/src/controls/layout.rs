use std::mem;
use std::ffi::{CStr, CString};
use libc::c_int;
use ui_sys::{self, uiBox, uiControl, uiGroup};
use super::Control;
use ui::UI;

/// Defines the ways in which the children of boxes can be layed out.
pub enum LayoutStrategy {
    /// Make the control the minimum possible size to contain its content
    Compact,
    /// Make the control expand to its maximum size
    Stretchy,
}

define_control! {
    /// A box that lays out its children vertically; see [`BoxExt`](trait.BoxExt.html) for functionality.
    rust_type: VerticalBox,
    sys_type: uiBox
}

define_control! {
    /// A box that lays out its children horizontally; see [`BoxExt`](trait.BoxExt.html) for functionality.
    rust_type: HorizontalBox,
    sys_type: uiBox
}

impl VerticalBox {
    /// Create a new vertical box layout.
    pub fn new(_ctx: &UI) -> VerticalBox {
        VerticalBox {
            uiBox: unsafe { ui_sys::uiNewVerticalBox() },
        }
    }
}

impl HorizontalBox {
    /// Create a new horizontal box layout.
    pub fn new(_ctx: &UI) -> VerticalBox {
        VerticalBox {
            uiBox: unsafe { ui_sys::uiNewHorizontalBox() },
        }
    }
}

fn append<T: Into<Control>>(b: *mut uiBox, ctx: &UI, child: T, strategy: LayoutStrategy) {
    let stretchy = match strategy {
        LayoutStrategy::Compact => false,
        LayoutStrategy::Stretchy => true,
    };
    let control = child.into();
    unsafe {
        assert!(ctx.parent_of(control.clone()).is_none());
        ui_sys::uiBoxAppend(b, control.ui_control, stretchy as c_int)
    }
}

fn padded(b: *mut uiBox, _ctx: &UI) -> bool {
    unsafe { ui_sys::uiBoxPadded(b) != 0 }
}

fn set_padded(b: *mut uiBox, padded: bool, _ctx: &UI) {
    unsafe { ui_sys::uiBoxSetPadded(b, padded as c_int) }
}

impl VerticalBox {
    /// Add a control to the end of the box, sized by the given layout strategy.
    pub fn append<T: Into<Control>>(&self, _ctx: &UI, child: T, strategy: LayoutStrategy) {
        append(self.uiBox, _ctx, child, strategy)
    }

    /// Determine whenther the box provides padding around its children.
    pub fn padded(&self, _ctx: &UI) -> bool {
        padded(self.uiBox, _ctx)
    }

    /// Set whether or not the box should provide padding around its children.
    pub fn set_padded(&self, _ctx: &UI, padded: bool) {
        set_padded(self.uiBox, padded, _ctx)
    }
}

impl HorizontalBox {
    /// Add a control to the end of the box, sized by the given layout strategy.
    pub fn append<T: Into<Control>>(&self, _ctx: &UI, child: T, strategy: LayoutStrategy) {
        append(self.uiBox, _ctx, child, strategy)
    }

    /// Determine whenther the box provides padding around its children.
    pub fn padded(&self, _ctx: &UI) -> bool {
        padded(self.uiBox, _ctx)
    }

    /// Set whether or not the box should provide padding around its children.
    pub fn set_padded(&self, _ctx: &UI, padded: bool) {
        set_padded(self.uiBox, padded, _ctx)
    }
}
