// Defines a new control, creating a Rust wrapper, a `Deref` implementation, and a destructor.
// An example of use:
//
//      define_control!{
//          /// Some documentation
//          #[attribute(whatever="something")]
//          control(Slider, uiSlider, ui_slider);
//      }
macro_rules! define_control {
    // Match first any attributes (incl. doc comments) and then the actual invocation
    {$(#[$attr:meta])* control($rust_type:ident, $ui_type:ident, $ui_field:ident);} => {
        // Include all attributes
        $(#[$attr])*
        pub struct $rust_type {
            $ui_field: *mut $ui_type,
        }

        impl ::std::ops::Deref for $rust_type {
            type Target = ::controls::Control;

            #[inline]
            fn deref(&self) -> &::controls::Control {
                // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
                unsafe {
                    mem::transmute::<&$rust_type, &::controls::Control>(self)
                }
            }
        }

        impl Drop for $rust_type {
            #[inline]
            fn drop(&mut self) {
                // For now this does nothing, but in the future, when `libui` supports proper
                // memory management, this will likely need to twiddle reference counts.
            }
        }

        impl Clone for $rust_type {
            #[inline]
            fn clone(&self) -> $rust_type {
                $rust_type {
                    $ui_field: self.$ui_field,
                }
            }
        }

        impl Into<Control> for $rust_type {
            #[inline]
            fn into(self) -> Control {
                unsafe {
                    let control = Control::from_ui_control(self.$ui_field as *mut uiControl);
                    mem::forget(self);
                    control
                }
            }
        }

        impl $rust_type {
            #[inline]
            pub unsafe fn from_ui_control($ui_field: *mut $ui_type) -> $rust_type {
                $rust_type {
                    $ui_field: $ui_field
                }
            }
        }
    }
}
