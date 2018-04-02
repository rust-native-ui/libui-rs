//! Low-level FFI bindings to `libui`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

extern crate libc;

use libc::{c_char, c_double, c_int, c_void, intmax_t, size_t, uintmax_t};

pub mod platform {
    pub mod macos;
    pub mod unix;
    pub mod windows;
}

#[repr(C)]
#[derive(Clone)]
pub struct uiInitOptions {
    pub Size: size_t,
}

extern {
    pub fn uiInit(options: *mut uiInitOptions) -> *const c_char;
    pub fn uiUninit();
    pub fn uiFreeInitError(err: *const c_char);

    pub fn uiMain();
    pub fn uiMainStep(wait: c_int) -> c_int;
    pub fn uiMainSteps();
    pub fn uiQuit();

    pub fn uiQueueMain(f: extern "C" fn(data: *mut c_void), data: *mut c_void);

    pub fn uiOnShouldQuit(f: extern "C" fn(data: *mut c_void), data: *mut c_void);

    pub fn uiFreeText(text: *mut c_char);
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct uiControl {
    pub Signature: u32,
    pub OSSignature: u32,
    pub TypeSignature: u32,
    pub Destroy: extern "C" fn(this: *mut uiControl),
    pub Handle: extern "C" fn(this: *mut uiControl) -> usize,
    pub Parent: extern "C" fn(this: *mut uiControl) -> *mut uiControl,
    pub SetParent: extern "C" fn(this: *mut uiControl, new_parent: *mut uiControl),
    pub Toplevel: extern "C" fn(this: *mut uiControl) -> c_int,
    pub Visible: extern "C" fn(this: *mut uiControl) -> c_int,
    pub Show: extern "C" fn(this: *mut uiControl),
    pub Hide: extern "C" fn(this: *mut uiControl),
    pub Enabled: extern "C" fn(this: *mut uiControl) -> c_int,
    pub Enable: extern "C" fn(this: *mut uiControl),
    pub Disable: extern "C" fn(this: *mut uiControl),
}

extern {
    pub fn uiControlDestroy(control: *mut uiControl);
    pub fn uiControlHandle(control: *mut uiControl) -> usize;
    pub fn uiControlParent(control: *mut uiControl) -> *mut uiControl;
    pub fn uiControlSetParent(control: *mut uiControl, new_parent: *mut uiControl);
    pub fn uiControlToplevel(control: *mut uiControl) -> c_int;
    pub fn uiControlVisible(control: *mut uiControl) -> c_int;
    pub fn uiControlShow(control: *mut uiControl);
    pub fn uiControlHide(control: *mut uiControl);
    pub fn uiControlEnabled(control: *mut uiControl) -> c_int;
    pub fn uiControlEnable(control: *mut uiControl);
    pub fn uiControlDisable(control: *mut uiControl);

    pub fn uiAllocControl(n: size_t, OSsig: u32, typesig: u32, typenamestr: *const c_char)
                          -> *mut uiControl;
    pub fn uiFreeControl(control: *mut uiControl);

    pub fn uiControlVerifyDestroy(control: *mut uiControl);
    pub fn uiControlVerifySetParent(control: *mut uiControl, new_parent: *mut uiControl);
    pub fn uiControlEnabledToUser(control: *mut uiControl) -> c_int;
}

pub enum uiWindow {}

extern {
    pub fn uiWindowTitle(w: *mut uiWindow) -> *mut c_char;
    pub fn uiWindowSetTitle(w: *mut uiWindow, title: *const c_char);
    pub fn uiWindowOnClosing(w: *mut uiWindow,
                             f: extern "C" fn(w: *mut uiWindow, data: *mut c_void) -> c_int,
                             data: *mut c_void);
    pub fn uiWindowSetChild(w: *mut uiWindow, child: *mut uiControl);
    pub fn uiWindowMargined(w: *mut uiWindow) -> c_int;
    pub fn uiWindowSetMargined(w: *mut uiWindow, margined: c_int);
    pub fn uiWindowSetAutosave(w: *mut uiWindow, name: *const c_char);
    pub fn uiNewWindow(title: *const c_char, width: c_int, height: c_int, hasMenubar: c_int)
                       -> *mut uiWindow;
}

pub enum uiButton {}

extern {
    pub fn uiButtonText(b: *mut uiButton) -> *mut c_char;
    pub fn uiButtonSetText(b: *mut uiButton, text: *const c_char);
    pub fn uiButtonOnClicked(b: *mut uiButton,
                             callback: extern "C" fn(b: *mut uiButton, data: *mut c_void),
                             data: *mut c_void);
    pub fn uiNewButton(text: *const c_char) -> *mut uiButton;
}

pub enum uiBox {}

extern {
    pub fn uiBoxAppend(b: *mut uiBox, child: *mut uiControl, stretchy: c_int);
    pub fn uiBoxDelete(b: *mut uiBox, index: uintmax_t);
    pub fn uiBoxPadded(b: *mut uiBox) -> c_int;
    pub fn uiBoxSetPadded(b: *mut uiBox, padded: c_int);
    pub fn uiNewHorizontalBox() -> *mut uiBox;
    pub fn uiNewVerticalBox() -> *mut uiBox;
}

pub enum uiEntry {}

extern {
    pub fn uiEntryText(e: *mut uiEntry) -> *mut c_char;
    pub fn uiEntrySetText(e: *mut uiEntry, text: *const c_char);
    pub fn uiEntryOnChanged(e: *mut uiEntry,
                            f: extern "C" fn(e: *mut uiEntry, data: *mut c_void),
                            data: *mut c_void);
    pub fn uiEntryReadOnly(e: *mut uiEntry) -> c_int;
    pub fn uiEntrySetReadOnly(e: *mut uiEntry, readonly: c_int);
    pub fn uiNewEntry() -> *mut uiEntry;
}

pub enum uiCheckbox {}

extern {
    pub fn uiCheckboxText(c: *mut uiCheckbox) -> *mut c_char;
    pub fn uiCheckboxSetText(c: *mut uiCheckbox, text: *const c_char);
    pub fn uiCheckboxOnToggled(c: *mut uiCheckbox,
                               f: extern "C" fn(c: *mut uiCheckbox, data: *mut c_void),
                               data: *mut c_void);
    pub fn uiCheckboxChecked(c: *mut uiCheckbox) -> c_int;
    pub fn uiCheckboxSetChecked(c: *mut uiCheckbox, checked: c_int);
    pub fn uiNewCheckbox(c: *const c_char) -> *mut uiCheckbox;
}

pub enum uiLabel {}

extern {
    pub fn uiLabelText(l: *mut uiLabel) -> *mut c_char;
    pub fn uiLabelSetText(l: *mut uiLabel, text: *const c_char);
    pub fn uiNewLabel(text: *const c_char) -> *mut uiLabel;
}

pub enum uiTab {}

extern {
    pub fn uiTabAppend(t: *mut uiTab, name: *const c_char, c: *mut uiControl);
    pub fn uiTabInsertAt(t: *mut uiTab, name: *const c_char, before: uintmax_t, c: *mut uiControl);
    pub fn uiTabDelete(t: *mut uiTab, index: uintmax_t);
    pub fn uiTabNumPages(t: *mut uiTab) -> uintmax_t;
    pub fn uiTabMargined(t: *mut uiTab, page: uintmax_t) -> c_int;
    pub fn uiTabSetMargined(t: *mut uiTab, page: uintmax_t, margined: c_int);
    pub fn uiNewTab() -> *mut uiTab;
}

pub enum uiGroup {}

extern {
    pub fn uiGroupTitle(g: *mut uiGroup) -> *mut c_char;
    pub fn uiGroupSetTitle(g: *mut uiGroup, title: *const c_char);
    pub fn uiGroupSetChild(g: *mut uiGroup, c: *mut uiControl);
    pub fn uiGroupMargined(g: *mut uiGroup) -> c_int;
    pub fn uiGroupSetMargined(g: *mut uiGroup, margined: c_int);
    pub fn uiNewGroup(title: *const c_char) -> *mut uiGroup;
}

pub enum uiSpinbox {}

extern {
    pub fn uiSpinboxValue(s: *mut uiSpinbox) -> intmax_t;
    pub fn uiSpinboxSetValue(s: *mut uiSpinbox, value: intmax_t);
    pub fn uiSpinboxOnChanged(s: *mut uiSpinbox,
                              f: extern "C" fn(s: *mut uiSpinbox, data: *mut c_void),
                              data: *mut c_void);
    pub fn uiNewSpinbox(min: intmax_t, max: intmax_t) -> *mut uiSpinbox;
}

pub enum uiProgressBar {}

extern {
    pub fn uiProgressBarSetValue(p: *mut uiProgressBar, n: c_int);
    pub fn uiNewProgressBar() -> *mut uiProgressBar;
}

pub enum uiSlider {}

extern {
    pub fn uiSliderValue(s: *mut uiSlider) -> intmax_t;
    pub fn uiSliderSetValue(s: *mut uiSlider, value: intmax_t);
    pub fn uiSliderOnChanged(s: *mut uiSlider,
                             f: extern "C" fn(s: *mut uiSlider, data: *mut c_void),
                             data: *mut c_void);
    pub fn uiNewSlider(min: intmax_t, max: intmax_t) -> *mut uiSlider;
}

pub enum uiSeparator {}

extern {
    pub fn uiNewHorizontalSeparator() -> *mut uiSeparator;
}

pub enum uiCombobox {}

extern {
    pub fn uiComboboxAppend(c: *mut uiCombobox, text: *const c_char);
    pub fn uiComboboxSelected(c: *mut uiCombobox) -> intmax_t;
    pub fn uiComboboxSetSelected(c: *mut uiCombobox, n: intmax_t);
    pub fn uiComboboxOnSelected(c: *mut uiCombobox,
                                f: extern "C" fn(c: *mut uiCombobox, data: *mut c_void),
                                data: *mut c_void);
    pub fn uiNewCombobox() -> *mut uiCombobox;
}

pub enum uiEditableCombobox {}

#[link(name = "ui")]
extern {
    pub fn uiNewEditableCombobox() -> *mut uiEditableCombobox;
    pub fn uiEditableComboboxAppend(c: *mut uiEditableCombobox, text: *const c_char);
}

pub enum uiRadioButtons {}

extern {
    pub fn uiRadioButtonsAppend(r: *mut uiRadioButtons, text: *const c_char);
    pub fn uiNewRadioButtons() -> *mut uiRadioButtons;
}

pub enum uiDateTimePicker {}

extern {
    pub fn uiNewDateTimePicker() -> *mut uiDateTimePicker;
    pub fn uiNewDatePicker() -> *mut uiDateTimePicker;
    pub fn uiNewTimePicker() -> *mut uiDateTimePicker;
}

pub enum uiMultilineEntry {}

extern {
    pub fn uiMultilineEntryText(e: *mut uiMultilineEntry) -> *mut c_char;
    pub fn uiMultilineEntrySetText(e: *mut uiMultilineEntry, text: *const c_char);
    pub fn uiMultilineEntryAppend(e: *mut uiMultilineEntry, text: *const c_char);
    pub fn uiMultilineEntryOnChanged(e: *mut uiMultilineEntry,
                                     f: extern "C" fn(e: *mut uiMultilineEntry, data: *mut c_void),
                                     data: *mut c_void);
    pub fn uiMultilineEntryReadOnly(e: *mut uiMultilineEntry) -> c_int;
    pub fn uiMultilineEntrySetReadOnly(e: *mut uiMultilineEntry, readonly: c_int);
    pub fn uiNewMultilineEntry() -> *mut uiMultilineEntry;
}

pub enum uiMenuItem {}

extern {
    pub fn uiMenuItemEnable(m: *mut uiMenuItem);
    pub fn uiMenuItemDisable(m: *mut uiMenuItem);
    pub fn uiMenuItemOnClicked(m: *mut uiMenuItem,
                               f: extern "C" fn(sender: *mut uiMenuItem,
                                                window: *mut uiWindow,
                                                data: *mut c_void),
                               data: *mut c_void);
    pub fn uiMenuItemChecked(m: *mut uiMenuItem) -> c_int;
    pub fn uiMenuItemSetChecked(m: *mut uiMenuItem, checked: c_int);
}

pub enum uiMenu {}

extern {
    pub fn uiMenuAppendItem(m: *mut uiMenu, name: *const c_char) -> *mut uiMenuItem;
    pub fn uiMenuAppendCheckItem(m: *mut uiMenu, name: *const c_char) -> *mut uiMenuItem;
    pub fn uiMenuAppendQuitItem(m: *mut uiMenu) -> *mut uiMenuItem;
    pub fn uiMenuAppendPreferencesItem(m: *mut uiMenu) -> *mut uiMenuItem;
    pub fn uiMenuAppendAboutItem(m: *mut uiMenu) -> *mut uiMenuItem;
    pub fn uiMenuAppendSeparator(m: *mut uiMenu);
    pub fn uiNewMenu(name: *const c_char) -> *mut uiMenu;
}

extern {
    pub fn uiOpenFile(parent: *mut uiWindow) -> *mut c_char;
    pub fn uiSaveFile(parent: *mut uiWindow) -> *mut c_char;
    pub fn uiMsgBox(parent: *mut uiWindow, title: *const c_char, description: *const c_char);
    pub fn uiMsgBoxError(parent: *mut uiWindow, title: *const c_char, description: *const c_char);
}

pub enum uiArea {}

pub enum uiDrawContext {}

#[repr(C)]
pub struct uiAreaHandler {
    pub Draw: extern "C" fn(this: *mut uiAreaHandler,
                            area: *mut uiArea,
                            draw_params: *mut uiAreaDrawParams),
    pub MouseEvent: extern "C" fn(this: *mut uiAreaHandler,
                                  area: *mut uiArea,
                                  mouse_event: *mut uiAreaMouseEvent),
    pub MouseCrossed: extern "C" fn(this: *mut uiAreaHandler, area: *mut uiArea, left: c_int),
    pub DragBroken: extern "C" fn(this: *mut uiAreaHandler, area: *mut uiArea),
    pub KeyEvent: extern "C" fn(this: *mut uiAreaHandler,
                                area: *mut uiArea,
                                key_event: *mut uiAreaKeyEvent)
                                -> c_int,
}

extern {
    pub fn uiAreaSetSize(a: *mut uiArea, width: intmax_t, height: intmax_t);
    pub fn uiAreaQueueRedrawAll(a: *mut uiArea);
    pub fn uiAreaScrollTo(a: *mut uiArea,
                          x: c_double,
                          y: c_double,
                          width: c_double,
                          height: c_double);
    pub fn uiNewArea(ah: *mut uiAreaHandler) -> *mut uiArea;
    pub fn uiNewScrollingArea(ah: *mut uiAreaHandler, width: intmax_t, height: intmax_t)
                              -> *mut uiArea;
}

#[repr(C)]
#[derive(Clone)]
pub struct uiAreaDrawParams {
    pub Context: *mut uiDrawContext,

    pub AreaWidth: c_double,
    pub AreaHeight: c_double,

    pub ClipX: c_double,
    pub ClipY: c_double,
    pub ClipWidth: c_double,
    pub ClipHeight: c_double,
}

pub enum uiDrawPath {}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum uiDrawBrushType {
    Solid = 0,
    LinearGradient = 1,
    RadialGradient = 2,
    Image = 3,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum uiDrawLineCap {
    Flat = 0,
    Round = 1,
    Square = 2,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum uiDrawLineJoin {
    Miter = 0,
    Round = 1,
    Bevel = 2,
}

pub const uiDrawDefaultMiterLimit: c_double = 10.0;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum uiDrawFillMode {
    Winding = 0,
    Alternate = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct uiDrawMatrix {
    pub M11: c_double,
    pub M12: c_double,
    pub M21: c_double,
    pub M22: c_double,
    pub M31: c_double,
    pub M32: c_double,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct uiDrawBrush {
    pub Type: uiDrawBrushType,

    // Solid brushes:
    pub R: c_double,
    pub G: c_double,
    pub B: c_double,
    pub A: c_double,

    // Gradient brushes:
    /// Linear: start X; radial: start X.
    pub X0: c_double,
    /// Linear: start Y; radial: start Y.
    pub Y0: c_double,
    /// Linear: end X; radial: outer circle center X.
    pub X1: c_double,
    /// Linear: end Y; radial: outer circle center Y.
    pub Y1: c_double,
    /// Radial gradients only.
    pub OuterRadius: c_double,
    pub Stops: *mut uiDrawBrushGradientStop,
    pub NumStops: size_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct uiDrawBrushGradientStop {
    pub Pos: c_double,
    pub R: c_double,
    pub G: c_double,
    pub B: c_double,
    pub A: c_double,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct uiDrawStrokeParams {
    pub Cap: uiDrawLineCap,
    pub Join: uiDrawLineJoin,
    pub Thickness: c_double,
    pub MiterLimit: c_double,
    pub Dashes: *mut c_double,
    pub NumDashes: size_t,
    pub DashPhase: c_double,
}

extern {
    pub fn uiDrawNewPath(fillMode: uiDrawFillMode) -> *mut uiDrawPath;
    pub fn uiDrawFreePath(p: *mut uiDrawPath);
}

extern {
    pub fn uiDrawPathNewFigure(p: *mut uiDrawPath, x: c_double, y: c_double);
    pub fn uiDrawPathNewFigureWithArc(p: *mut uiDrawPath,
                                      xCenter: c_double,
                                      yCenter: c_double,
                                      radius: c_double,
                                      startAngle: c_double,
                                      sweep: c_double,
                                      negative: c_int);
    pub fn uiDrawPathLineTo(p: *mut uiDrawPath, x: c_double, y: c_double);
    pub fn uiDrawPathArcTo(p: *mut uiDrawPath,
                           xCenter: c_double,
                           yCenter: c_double,
                           radius: c_double,
                           startAngle: c_double,
                           sweep: c_double,
                           negative: c_int);
    pub fn uiDrawPathBezierTo(p: *mut uiDrawPath,
                              c1x: c_double,
                              c1y: c_double,
                              c2x: c_double,
                              c2y: c_double,
                              endX: c_double,
                              endY: c_double);
    pub fn uiDrawPathCloseFigure(p: *mut uiDrawPath);

    pub fn uiDrawPathAddRectangle(p: *mut uiDrawPath,
                                  x: c_double,
                                  y: c_double,
                                  width: c_double,
                                  height: c_double);
    pub fn uiDrawPathEnd(p: *mut uiDrawPath);

    pub fn uiDrawStroke(c: *mut uiDrawContext,
                        path: *mut uiDrawPath,
                        b: *mut uiDrawBrush,
                        p: *mut uiDrawStrokeParams);
    pub fn uiDrawFill(c: *mut uiDrawContext, path: *mut uiDrawPath, b: *mut uiDrawBrush);
}

extern "C" {
    pub fn uiDrawMatrixSetIdentity(m: *mut uiDrawMatrix);
    pub fn uiDrawMatrixTranslate(m: *mut uiDrawMatrix, x: c_double, y: c_double);
    pub fn uiDrawMatrixScale(m: *mut uiDrawMatrix,
                             xCenter: c_double,
                             yCenter: c_double,
                             x: c_double,
                             y: c_double);
    pub fn uiDrawMatrixRotate(m: *mut uiDrawMatrix, x: c_double, y: c_double, amount: c_double);
    pub fn uiDrawMatrixSkew(m: *mut uiDrawMatrix,
                            x: c_double,
                            y: c_double,
                            xamount: c_double,
                            yamount: c_double);
    pub fn uiDrawMatrixMultiply(dest: *mut uiDrawMatrix, src: *mut uiDrawMatrix);
    pub fn uiDrawMatrixInvertible(m: *mut uiDrawMatrix) -> c_int;
    pub fn uiDrawMatrixInvert(m: *mut uiDrawMatrix) -> c_int;
    pub fn uiDrawMatrixTransformPoint(m: *mut uiDrawMatrix, x: *mut c_double, y: *mut c_double);
    pub fn uiDrawMatrixTransformSize(m: *mut uiDrawMatrix, x: *mut c_double, y: *mut c_double);
}

extern "C" {
    pub fn uiDrawTransform(c: *mut uiDrawContext, m: *mut uiDrawMatrix);

    pub fn uiDrawClip(c: *mut uiDrawContext, path: *mut uiDrawPath);

    pub fn uiDrawSave(c: *mut uiDrawContext);
    pub fn uiDrawRestore(c: *mut uiDrawContext);
}

pub enum uiDrawFontFamilies {}

extern "C" {
    pub fn uiDrawListFontFamilies() -> *mut uiDrawFontFamilies;
    pub fn uiDrawFontFamiliesNumFamilies(ff: *mut uiDrawFontFamilies) -> uintmax_t;
    pub fn uiDrawFontFamiliesFamily(ff: *mut uiDrawFontFamilies, n: uintmax_t) -> *mut c_char;
    pub fn uiDrawFreeFontFamilies(ff: *mut uiDrawFontFamilies);
}

pub enum uiDrawTextLayout {}
pub enum uiDrawTextFont {}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum uiDrawTextWeight {
    Thin = 0,
    UltraLight = 1,
    Light = 2,
    Book = 3,
    Normal = 4,
    Medium = 5,
    SemiBold = 6,
    Bold = 7,
    /// [sic]
    UtraBold = 8,
    Heavy = 9,
    UltraHeavy = 10,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum uiDrawTextItalic {
    Normal = 0,
    Oblique = 1,
    Italic = 2,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum uiDrawTextStretch {
    UltraCondensed = 0,
    ExtraCondensed = 1,
    Condensed = 2,
    SemiCondensed = 3,
    Normal = 4,
    SemiExpanded = 5,
    Expanded = 6,
    ExtraExpanded = 7,
    UltraExpanded = 8,
}

#[repr(C)]
#[derive(Clone)]
pub struct uiDrawTextFontDescriptor {
    pub Family: *const c_char,
    pub Size: c_double,
    pub Weight: uiDrawTextWeight,
    pub Italic: uiDrawTextItalic,
    pub Stretch: uiDrawTextStretch,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct uiDrawTextFontMetrics {
    pub Ascent: c_double,
    pub Descent: c_double,
    pub Leading: c_double,
    pub UnderlinePos: c_double,
    pub UnderlineThickness: c_double,
}

extern "C" {
    pub fn uiDrawLoadClosestFont(desc: *const uiDrawTextFontDescriptor) -> *mut uiDrawTextFont;
    pub fn uiDrawFreeTextFont(font: *mut uiDrawTextFont);
    pub fn uiDrawTextFontHandle(font: *mut uiDrawTextFont) -> usize;
    pub fn uiDrawTextFontDescribe(font: *mut uiDrawTextFont,
                                  desc: *mut uiDrawTextFontDescriptor);
    pub fn uiDrawTextFontGetMetrics(font: *mut uiDrawTextFont,
                                    metrics: *mut uiDrawTextFontMetrics);
}

extern "C" {
    pub fn uiDrawNewTextLayout(text: *const c_char,
                               defaultFont: *mut uiDrawTextFont,
                               width: c_double)
                               -> *mut uiDrawTextLayout;
    pub fn uiDrawFreeTextLayout(layout: *mut uiDrawTextLayout);
    pub fn uiDrawTextLayoutSetWidth(layout: *mut uiDrawTextLayout, width: c_double);
    pub fn uiDrawTextLayoutExtents(layout: *mut uiDrawTextLayout,
                                   width: *mut c_double,
                                   height: *mut c_double);

    pub fn uiDrawTextLayoutSetColor(layout: *mut uiDrawTextLayout,
                                    startChar: intmax_t,
                                    endChar: intmax_t,
                                    r: c_double,
                                    g: c_double,
                                    b: c_double,
                                    a: c_double);

    pub fn uiDrawText(c: *mut uiDrawContext,
                      x: c_double,
                      y: c_double,
                      layout: *mut uiDrawTextLayout);
}

pub type uiModifiers = c_int;

pub const uiModifierCtrl: uiModifiers = 1 << 0;
pub const uiModifierAlt: uiModifiers = 1 << 1;
pub const uiModifierShift: uiModifiers = 1 << 2;
pub const uiModifierSuper: uiModifiers = 1 << 3;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct uiAreaMouseEvent {
    pub X: c_double,
    pub Y: c_double,

    pub AreaWidth: c_double,
    pub AreaHeight: c_double,

    pub Down: c_int,
    pub Up: c_int,

    pub Count: c_int,

    pub Modifiers: uiModifiers,

    pub Held1To64: u64,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum uiExtKey {
    Escape = 1,
    /// Equivalent to "Help" on Apple keyboards.
    Insert = 2,
    Delete = 3,
    Home = 4,
    End = 5,
    PageUp = 6,
    PageDown = 7,
    Up = 8,
    Down = 9,
    Left = 10,
    Right = 11,
    // F1..F12 are guaranteed to be consecutive.
    F1 = 12,
    F2 = 13,
    F3 = 14,
    F4 = 15,
    F5 = 16,
    F6 = 17,
    F7 = 18,
    F8 = 19,
    F9 = 20,
    F10 = 21,
    F11 = 22,
    F12 = 23,
    // Numpad keys; independent of Num Lock state.
    // N0..N9 are guaranteed to be consecutive.
    N0 = 24,
    N1 = 25,
    N2 = 26,
    N3 = 27,
    N4 = 28,
    N5 = 29,
    N6 = 30,
    N7 = 31,
    N8 = 32,
    N9 = 33,
    NDot = 34,
    NEnter = 35,
    NAdd = 36,
    NSubtract = 37,
    NMultiply = 38,
    NDivide = 39,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct uiAreaKeyEvent {
    pub Key: c_char,
    pub ExtKey: uiExtKey,
    pub Modifier: uiModifiers,

    pub Modifiers: uiModifiers,

    pub Up: c_int,
}

pub enum uiFontButton {}

extern {
    pub fn uiFontButtonFont(b: *mut uiFontButton) -> *mut uiDrawTextFont;
    pub fn uiFontButtonOnChanged(b: *mut uiFontButton,
                                 f: extern "C" fn(this: *mut uiFontButton, data: *mut c_void),
                                 data: *mut c_void);
    pub fn uiNewFontButton() -> *mut uiFontButton;
}

pub enum uiColorButton {}

extern {
    pub fn uiColorButtonColor(b: *mut uiColorButton,
                              r: *mut c_double,
                              g: *mut c_double,
                              bl: *mut c_double,
                              a: *mut c_double);
    pub fn uiColorButtonSetColor(b: *mut uiColorButton,
                                 r: c_double,
                                 g: c_double,
                                 bl: c_double,
                                 a: c_double);
    pub fn uiColorButtonOnChanged(b: *mut uiColorButton,
                                  f: extern "C" fn(this: *mut uiColorButton, data: *mut c_void),
                                  data: *mut c_void);
    pub fn uiNewColorButton() -> *mut uiColorButton;
}


pub enum uiPixmapImage {}
pub type uiPixmap32Format = u32;

#[repr(C)]
pub struct uiImageData {
    pub fmt: uiPixmap32Format,
    pub width: c_int,
    pub height: c_int,
    pub rowstride: c_int,
    pub data: *mut c_void,
}

extern {
    pub fn uiNewPixmapImage(width: c_int, height: c_int) -> *mut uiPixmapImage;
    pub fn uiFreePixmapImage(img: *mut uiPixmapImage);
    pub fn uiPixmapImageGetFormat(img: *mut uiPixmapImage) -> uiPixmap32Format;
    pub fn uiPixmapImageGetData(img: *const uiPixmapImage, data: *mut uiImageData);
    pub fn uiImageLoadPixmap32Raw(img: *mut uiPixmapImage,
                                  x: c_int,
                                  y: c_int,
                                  width: c_int,
                                  height: c_int,
                                  rowstrideBytes: c_int,
                                  fmt: uiPixmap32Format,
                                  data: *const c_void);
    pub fn uiDrawPixmapImage(c: *mut uiDrawContext,
                             x: c_double,
                             y: c_double,
                             img: *const uiPixmapImage);

    pub fn uiScalePixmapImage(c: *mut uiDrawContext,
                             xScale: c_double,
                             yScale: c_double);
}
