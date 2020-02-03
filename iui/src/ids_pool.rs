//! Macros in this module provide an easy way to manipulate
//! ui controls with their ids.
//!
//!  # Example
//!
//!  ```
//! #[macro_use]
//! use iui::preclude::*;
//! use iui::ids_pool;
//! use std::cell::RefCell;
//!
//! //Create the handle of libui 
//! thread_local! {
//!     pub static UI:RefCell<iui::UI> 
//!         = RefCell::new(iui::UI::init().expect("Couldn't initialize UI library"));
//! }
//! 
//! //Provide a thread local ui controls ids pool
//! thread_ui_ids_pool!(
//!     Button,
//!     Checkbox,	
//!     Combobox,
//!     Entry,
//!     Group,
//!     HorizontalBox,
//!     HorizontalSeparator,
//!     Label,
//!     LayoutGrid,
//!     MultilineEntry,
//!     Slider,
//!     Spacer,
//!     Spinbox,
//!     TabGroup,
//!     VerticalBox,
//!     Window);
//!
//! fn build_master_tab_info_group(){
//!     let ui =  UI.with(|u| {(*u.borrow()).clone()});
//!     ui_new!(master_info_group,Group,&ui, "master info");
//!     ui_new!(master_info_group_vbox,VerticalBox,&ui);
//!     master_info_group_vbox.set_padded(&ui, true);
//!
//!     ui_new!(master_layout_grid1,LayoutGrid,&ui);
//!     ui_new!(label_master_hw_type_,Label,&ui, "system type:");
//!     master_layout_grid1.append(&ui, label_master_hw_type_.clone(),
//!     0,0,1,1,GridExpand::Neither,GridAlignment::Fill,GridAlignment::Fill);
//! 
//!     ui_new!(label_master_hw_type,Label,&ui, "xxx");
//!     master_layout_grid1.append(&ui, label_master_hw_type.clone(),
//!     1,0,1,1,GridExpand::Horizontal,GridAlignment::Fill,GridAlignment::Fill)
//! 
//!     master_info_group_vbox.append(&ui,master_layout_grid1.clone(),LayoutStrategy::Compact);
//!     master_info_group.set_child(&ui,master_info_group_vbox.clone());
//!     ui_must_get!(master_vbox,VerticalBox).append(&ui, master_info_group.clone(),LayoutStrategy::Compact);
//! }
//!
//! fn build_master_tab_level_config_group(){
//!     let ui =  UI.with(|u| {(*u.borrow()).clone()});
//!     ui_new!(master_level_config_group,Group,&ui, "master setting1");
//!     ui_new!(master_level_config_group_vbox,VerticalBox,&ui);
//!     master_level_config_group_vbox.set_padded(&ui, true);
//!     ui_new!(master_layout_grid2,LayoutGrid,&ui);
//!     ui_new!(label_master_lv_channel_,Label,&ui, " disk: ");
//!     master_layout_grid2.append(&ui, label_master_lv_channel_.clone(),
//!     0,1,1,1,GridExpand::Neither,GridAlignment::Fill,GridAlignment::Fill);
//! 
//!     ui_new!(entry_master_lv_channel,Entry,&ui);
//!     master_layout_grid2.append(&ui, entry_master_lv_channel.clone(),
//!     1,1,1,1,GridExpand::Horizontal,GridAlignment::Fill,GridAlignment::Fill);
//! 
//!     master_level_config_group_vbox.append(&ui,master_layout_grid3,LayoutStrategy::Compact);
//!     master_level_config_group.set_child(&ui,master_level_config_group_vbox.clone());
//!     ui_must_get!(master_vbox,VerticalBox)
//!         .append(&ui, master_level_config_group.clone(),LayoutStrategy::Compact);
//! }
//!
//! fn build_master_tab_apple_config_group(){
//!     let ui =  UI.with(|u| {(*u.borrow()).clone()});
//!     ui_new!(master_apple_config_group,Group,&ui, "master setting2");
//!     ui_new!(master_apple_config_group_vbox,VerticalBox,&ui);
//!     master_apple_config_group_vbox.set_padded(&ui, true);
//! 
//!     ui_new!(label_master_apple_baudrate_,Label,&ui, " memory: ");
//!     master_layout_grid4.append(&ui, label_master_apple_baudrate_.clone(),
//!     0,1,1,1,GridExpand::Neither,GridAlignment::Fill,GridAlignment::Fill);
//! 
//!     ui_new!(entry_master_apple_baudrate,Entry,&ui);
//!     master_layout_grid4.append(&ui, entry_master_apple_baudrate.clone(),
//!     1,1,1,1,GridExpand::Horizontal,GridAlignment::Fill,GridAlignment::Fill);
//! 
//!     master_apple_config_group_vbox.append(&ui,master_layout_grid5,LayoutStrategy::Compact);
//!     master_apple_config_group.set_child(&ui,master_apple_config_group_vbox.clone());
//!     ui_must_get!(master_vbox,VerticalBox)
//!     .append(&ui, master_apple_config_group.clone(),LayoutStrategy::Compact);
//! }
//!
//! fn build_master_tab(){
//!     let ui =  UI.with(|u| {(*u.borrow()).clone()});
//!     // Create a vertical layout to hold the controls
//!     ui_new!(master_vbox,VerticalBox,&ui);
//!     master_vbox.set_padded(&ui, true);
//! 
//!     ui_new!(master_tip_label,Label,&ui, "master starting...");
//!     master_vbox.append(&ui, master_tip_label.clone(), LayoutStrategy::Compact);
//!     build_master_tab_info_group();
//!     build_master_tab_level_config_group();
//!     build_master_tab_apple_config_group();
//!     let mut tab_group = ui_must_get!(tab_group,TabGroup);
//!     tab_group.append(&ui,"master",master_vbox);
//! }
//!
//! fn build_slave_tab(){
//!     let ui =  UI.with(|u| {(*u.borrow()).clone()});
//!     ui_new!(slave_vbox,VerticalBox,&ui);
//!     ui_new!(slave_tip_label,Label,&ui, "slave system starting...");
//!     slave_vbox.append(&ui, slave_tip_label.clone(), LayoutStrategy::Compact);
//!     let mut tab_group = ui_must_get!(tab_group,TabGroup);
//!     tab_group.append(&ui,"slave",slave_vbox);
//! }
//! 
//! pub fn show_ui() {
//!     debug!("starting ui");
//!     let ui =  UI.with(|u| {(*u.borrow()).clone()});
//!     ui_new!(win,Window,&ui,"Hello World V0.0", 200, 200, WindowType::NoMenubar);
//!     win.set_margined(&ui, true);
//! 
//!     ui_new!(tab_group,TabGroup,&ui);
//! 
//!     build_master_tab();
//!     build_slave_tab();
//! 
//!     // Actually put the button in the window
//!     win.set_child(&ui, tab_group);
//!     // Show the window
//!     win.show(&ui);
//!     // Run the application
//!     ui.main();
//! }
//! ```

/// This macro create a ui control, and put the "id" into a thread local
/// pool called UI_IDS_POOL if the "id" does not exists in the pool
/// The first argument is the "id", you should make it unique.
/// The second argument is the control type.
/// The left arguments are used to call with the control's new() function.
#[allow(unused_macros)]
macro_rules! ui_new{
    ( $id:ident,$control_type:ident) => (
        ui_new!($id,$control_type,)
    );
    ( $id:ident,$control_type:ident,$($new_arg:tt)*) => (
            #[allow(unused_mut)]
            let mut $id= $control_type::new($($new_arg)*);
            {
                let cloned_id = $id.clone();
                UI_IDS_POOL.with(move |p| {
                    let h =  &mut p.borrow_mut().$control_type;
                    if !h.contains_key(stringify!($id)){
                        h.insert(stringify!($id).to_owned(),cloned_id);
                    }else{
                        error!("ui id:{} already exists.",stringify!($id));
                    }
                })
            }
    );
}

/// This macro get a control from UI_IDS_POOL
/// The first argument is the "id" of the control you want to get it from pool.
/// The second argument is the control's type.
/// The return type is Option<control's type>
#[allow(unused_macros)]
macro_rules! ui_get{
    ( $id:ident, $control_type:expr) => {
        UI_IDS_POOL.with(|p| {
            let h =  &p.borrow().$control_type;
            h.get(stringify!($id)).cloned()
        })
    }
}

/// This macro get a control from UI_IDS_POOL.If the id does not exists in the
/// pool, it panics.
/// The first argument is the "id" of the control you want to get it from pool.
/// The second argument is the control type.
/// The return type is the control's type
#[allow(unused_macros)]
macro_rules! ui_must_get{
    ( $id:ident, $control_type:tt) => {
        UI_IDS_POOL.with(|p| {
            let h =  &p.borrow().$control_type;
            h.get(stringify!($id)).cloned().unwrap()
        })
    }
}

/// This macro build an ids pool named UI_IDS_POOL.
/// The arguments control types.
macro_rules! thread_ui_ids_pool{
    ($($control:ident),*) => {
        #[allow(non_snake_case)]
        #[derive(Clone)]
        pub struct UiIdsPool{
            $(pub $control:std::collections::HashMap<String,$control>,)*
        }

        impl UiIdsPool{
            pub fn new()->Self{
                UiIdsPool{
                    $($control:std::collections::HashMap::new(),)*
                }
            }
        }

        thread_local! {
            pub static UI_IDS_POOL:std::cell::RefCell<UiIdsPool> = RefCell::new(UiIdsPool::new());
        }
    };
}


