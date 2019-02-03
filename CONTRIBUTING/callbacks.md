# Callbacks with libui

libui provides a mechanism through which each callback can be passed arbitrary data. (An
untyped buffer.)
IUI uses this mechanism to provide a C "wrapper" function which converts any raw libui
types into IUI Rust types, and does sanity checking.

We do this with the functions `callback_helpers::to_heap_ptr`, which `Box`es up a Rust
type and returns a pointer to the allocated memory, and `callback_helpers::from_void_ptr`,
which reconstitutes the Rust type (specified using generics). These functions are `unsafe`
for obvious reasons, and can also leak memory if used improperly.

Their intended use is for `to_heap_ptr` to turn a Rust function into a bag of bits and for
`from_void_ptr` to reconstitute that function into the _exact same type_, which we ensure
by "generic locking" the user and wrapper functions, like so:

```rust
fn on_whatever<'ctx, F: FnMut(&Whatever) + 'static>(&mut self, _ctx: &'ctx UI, callback: F) {

    fn c_callback<G: FnMut(&Whatever)> { /* ... do stuff ... */ }

    ui_sys::uiWhateverOnWhatever(/* ... */, c_callback::<F>);
}
```

This is somewhat verbose but ensures that the types do not deviate, which would be unsafe.

Callbacks should be named `on_event` where `event` is, for instance, `clicked` or
`closing`. The functions taken by callbacks must always have the `'static` bound.

