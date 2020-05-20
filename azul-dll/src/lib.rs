// WARNING: autogenerated code for azul api version 0.1.0
//! Public API for Azul
//!
//! A single function can have multiple implementations depending on whether it is
//! compiled for the Rust-desktop target, the Rust-wasm target or the C API.
//!
//! For now, the crate simply re-exports azul_core and calls the c_api functions

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/maps4print/azul/master/assets/images/azul_logo_full_min.svg.png",
    html_favicon_url = "https://raw.githubusercontent.com/maps4print/azul/master/assets/images/favicon.ico",
)]

#![allow(dead_code)]
#![allow(unused_imports)]

extern crate azul_core;
extern crate azul_css;
extern crate azul_native_style;
#[cfg(target_arch = "wasm32")]
extern crate azul_web;
#[cfg(not(target_arch = "wasm32"))]
extern crate azul_desktop;

use core::ffi::c_void;
use azul_core::dom::Dom;
use azul_core::callbacks::{RefAny, LayoutInfo};
use azul_css::Css;
use azul_core::window::WindowCreateOptions;
#[cfg(not(target_arch = "wasm32"))]
use azul_desktop::app::{App, AppConfig};
#[cfg(target_arch = "wasm32")]
use azul_web::app::{App, AppConfig};

/// The layout() callback fn
pub type AzLayoutCallback = fn(AzRefAny, AzLayoutInfoPtr) -> AzDomPtr;


/// Pointer to rust-allocated `Box<String>` struct
#[no_mangle] #[repr(C)] pub struct AzStringPtr { ptr: *mut c_void }
/// Creates + allocates a Rust `String` by **copying** it from another utf8-encoded string
#[no_mangle] pub extern "C" fn az_string_from_utf8_unchecked(ptr: *const u8, len: usize) -> AzStringPtr { let object: String = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len)).to_string() }; AzStringPtr { ptr: Box::into_raw(Box::new(object)) as *mut c_void } }
/// Creates + allocates a Rust `String` by **copying** it from another utf8-encoded string
#[no_mangle] pub extern "C" fn az_string_from_utf8_lossy(ptr: *const u8, len: usize) -> AzStringPtr { let object: String = unsafe { std::string::String::from_utf8_lossy(std::slice::from_raw_parts(ptr, len)).to_string() }; AzStringPtr { ptr: Box::into_raw(Box::new(object)) as *mut c_void } }
/// Destructor: Takes ownership of the `String` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_string_delete(ptr: &mut AzStringPtr) { let _ = unsafe { Box::<String>::from_raw(ptr.ptr  as *mut String) }; }
/// Copies the pointer: WARNING: After calling this function you'll have two pointers to the same Box<`String`>!.
#[no_mangle] pub extern "C" fn az_string_shallow_copy(ptr: &AzStringPtr) -> AzStringPtr { AzStringPtr { ptr: ptr.ptr } }
/// (private): Downcasts the `AzStringPtr` to a `Box<String>`. Note that this takes ownership of the pointer.
fn az_string_downcast(ptr: AzStringPtr) -> Box<String> { unsafe { Box::<String>::from_raw(ptr.ptr  as *mut String) } }

/// Pointer to rust-allocated `Box<AppConfig>` struct
#[no_mangle] #[repr(C)] pub struct AzAppConfigPtr { ptr: *mut c_void }
// Creates a new `AppConfig` instance whose memory is owned by the rust allocator
// Equivalent to the Rust `AppConfig::new()` constructor.
#[no_mangle] pub extern "C" fn az_app_config_new() -> AzAppConfigPtr { let object: AppConfig = AppConfig::default(); AzAppConfigPtr { ptr: Box::into_raw(Box::new(object)) as *mut c_void } }
/// Destructor: Takes ownership of the `AppConfig` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_app_config_delete(ptr: &mut AzAppConfigPtr) { let _ = unsafe { Box::<AppConfig>::from_raw(ptr.ptr  as *mut AppConfig) }; }
/// Copies the pointer: WARNING: After calling this function you'll have two pointers to the same Box<`AppConfig`>!.
#[no_mangle] pub extern "C" fn az_app_config_shallow_copy(ptr: &AzAppConfigPtr) -> AzAppConfigPtr { AzAppConfigPtr { ptr: ptr.ptr } }
/// (private): Downcasts the `AzAppConfigPtr` to a `Box<AppConfig>`. Note that this takes ownership of the pointer.
fn az_app_config_downcast(ptr: AzAppConfigPtr) -> Box<AppConfig> { unsafe { Box::<AppConfig>::from_raw(ptr.ptr  as *mut AppConfig) } }

/// Pointer to rust-allocated `Box<App>` struct
#[no_mangle] #[repr(C)] pub struct AzAppPtr { ptr: *mut c_void }
/// Creates a new App instance.
#[no_mangle] pub extern "C" fn az_app_new(data: AzRefAny, config: AzAppConfigPtr, callback: AzLayoutCallback) -> AzAppPtr { let object: App = App::new(data, *az_app_config_downcast(config), callback).unwrap(); AzAppPtr { ptr: Box::into_raw(Box::new(object)) as *mut c_void } }
// Equivalent to the Rust `App::run()` function.
#[no_mangle] pub extern "C" fn az_app_run(app: AzAppPtr, window: AzWindowCreateOptionsPtr) { az_app_downcast(app).run(*az_window_create_options_downcast(window)) }
/// Destructor: Takes ownership of the `App` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_app_delete(ptr: &mut AzAppPtr) { let _ = unsafe { Box::<App>::from_raw(ptr.ptr  as *mut App) }; }
/// Copies the pointer: WARNING: After calling this function you'll have two pointers to the same Box<`App`>!.
#[no_mangle] pub extern "C" fn az_app_shallow_copy(ptr: &AzAppPtr) -> AzAppPtr { AzAppPtr { ptr: ptr.ptr } }
/// (private): Downcasts the `AzAppPtr` to a `Box<App>`. Note that this takes ownership of the pointer.
fn az_app_downcast(ptr: AzAppPtr) -> Box<App> { unsafe { Box::<App>::from_raw(ptr.ptr  as *mut App) } }

/// Pointer to rust-allocated `Box<RefAny>` struct
pub use ::azul_core::callbacks::RefAny as AzRefAny;

/// Creates a new `RefAny` instance
#[no_mangle] pub extern "C" fn az_ref_any_new(ptr: *const u8, len: usize, type_id: u64, type_name: AzStringPtr, custom_destructor: fn(AzRefAny)) -> AzRefAny {
    AzRefAny::new_c(ptr, len, type_id, *az_string_downcast(type_name), custom_destructor)
}
/// Returns the internal pointer of the `RefAny` as a `*mut c_void` or a nullptr if the types don't match
#[no_mangle] pub extern "C" fn az_ref_any_get_ptr(ptr: &AzRefAny, len: usize, type_id: u64) -> *const c_void { ptr.get_ptr(len, type_id) }
/// Returns the internal pointer of the `RefAny` as a `*mut c_void` or a nullptr if the types don't match
#[no_mangle] pub extern "C" fn az_ref_any_get_mut_ptr(ptr: &AzRefAny, len: usize, type_id: u64) -> *mut c_void { ptr.get_mut_ptr(len, type_id) }
/// Creates a new reference of the pointer, pointing to the same object: WARNING: After calling this function you'll have two pointers to the same Box<`RefAny`>!.
#[no_mangle] pub extern "C" fn az_ref_any_shallow_copy(ptr: &AzRefAny) -> AzRefAny { ptr.clone() }
/// Destructor: Takes ownership of the `RefAny` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_ref_any_delete(ptr: &mut AzRefAny) { az_ref_any_core_copy(ptr).drop_c() }
/// Copies the pointer without invoking the destructor
#[no_mangle] pub extern "C" fn az_ref_any_core_copy(ptr: &AzRefAny) -> AzRefAny {
    AzRefAny {
        _internal_ptr: ptr._internal_ptr,
        _internal_len: ptr._internal_len,
        _internal_layout_size: ptr._internal_layout_size,
        _internal_layout_align: ptr._internal_layout_align,
        type_id: ptr.type_id,
        type_name: ptr.type_name.clone(),
        strong_count: ptr.strong_count,
        is_currently_mutable: ptr.is_currently_mutable,
        custom_destructor: ptr.custom_destructor,
    }
}

/// Pointer to rust-allocated `Box<LayoutInfo>` struct
pub use ::azul_core::callbacks::LayoutInfoPtr as AzLayoutInfoPtr;
/// Destructor: Takes ownership of the `LayoutInfo` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_layout_info_delete<'a>(ptr: &mut AzLayoutInfoPtr) { let _ = unsafe { Box::<LayoutInfo<'a>>::from_raw(ptr.ptr  as *mut LayoutInfo<'a>) }; }
/// Copies the pointer: WARNING: After calling this function you'll have two pointers to the same Box<`LayoutInfo`>!.
#[no_mangle] pub extern "C" fn az_layout_info_shallow_copy<'a>(ptr: &AzLayoutInfoPtr) -> AzLayoutInfoPtr { AzLayoutInfoPtr { ptr: ptr.ptr } }
/// (private): Downcasts the `AzLayoutInfoPtr` to a `Box<LayoutInfo<'a>>`. Note that this takes ownership of the pointer.
fn az_layout_info_downcast<'a>(ptr: AzLayoutInfoPtr) -> Box<LayoutInfo<'a>> { unsafe { Box::<LayoutInfo<'a>>::from_raw(ptr.ptr  as *mut LayoutInfo<'a>) } }

/// Pointer to rust-allocated `Box<Dom>` struct
pub use ::azul_core::dom::DomPtr as AzDomPtr;
// Creates a new `Dom` instance whose memory is owned by the rust allocator
// Equivalent to the Rust `Dom::div()` constructor.
#[no_mangle] pub extern "C" fn az_dom_div() -> AzDomPtr { let object: Dom = Dom::div(); AzDomPtr { ptr: Box::into_raw(Box::new(object)) as *mut c_void } }
// Creates a new `Dom` instance whose memory is owned by the rust allocator
// Equivalent to the Rust `Dom::label()` constructor.
#[no_mangle] pub extern "C" fn az_dom_label(text: AzStringPtr) -> AzDomPtr { let object: Dom = Dom::label(*az_string_downcast(text)); AzDomPtr { ptr: Box::into_raw(Box::new(object)) as *mut c_void } }
/// Destructor: Takes ownership of the `Dom` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_dom_delete(ptr: &mut AzDomPtr) { let _ = unsafe { Box::<Dom>::from_raw(ptr.ptr  as *mut Dom) }; }
/// Copies the pointer: WARNING: After calling this function you'll have two pointers to the same Box<`Dom`>!.
#[no_mangle] pub extern "C" fn az_dom_shallow_copy(ptr: &AzDomPtr) -> AzDomPtr { AzDomPtr { ptr: ptr.ptr } }
/// (private): Downcasts the `AzDomPtr` to a `Box<Dom>`. Note that this takes ownership of the pointer.
fn az_dom_downcast(ptr: AzDomPtr) -> Box<Dom> { unsafe { Box::<Dom>::from_raw(ptr.ptr  as *mut Dom) } }

/// Pointer to rust-allocated `Box<Css>` struct
#[no_mangle] #[repr(C)] pub struct AzCssPtr { ptr: *mut c_void }
// Creates a new `Css` instance whose memory is owned by the rust allocator
// Equivalent to the Rust `Css::native()` constructor.
#[no_mangle] pub extern "C" fn az_css_native() -> AzCssPtr { let object: Css = azul_native_style::native(); AzCssPtr { ptr: Box::into_raw(Box::new(object)) as *mut c_void } }
/// Destructor: Takes ownership of the `Css` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_css_delete(ptr: &mut AzCssPtr) { let _ = unsafe { Box::<Css>::from_raw(ptr.ptr  as *mut Css) }; }
/// Copies the pointer: WARNING: After calling this function you'll have two pointers to the same Box<`Css`>!.
#[no_mangle] pub extern "C" fn az_css_shallow_copy(ptr: &AzCssPtr) -> AzCssPtr { AzCssPtr { ptr: ptr.ptr } }
/// (private): Downcasts the `AzCssPtr` to a `Box<Css>`. Note that this takes ownership of the pointer.
fn az_css_downcast(ptr: AzCssPtr) -> Box<Css> { unsafe { Box::<Css>::from_raw(ptr.ptr  as *mut Css) } }

/// Pointer to rust-allocated `Box<WindowCreateOptions>` struct
#[no_mangle] #[repr(C)] pub struct AzWindowCreateOptionsPtr { ptr: *mut c_void }
// Creates a new `WindowCreateOptions` instance whose memory is owned by the rust allocator
// Equivalent to the Rust `WindowCreateOptions::new()` constructor.
#[no_mangle] pub extern "C" fn az_window_create_options_new(css: AzCssPtr) -> AzWindowCreateOptionsPtr { let object: WindowCreateOptions = WindowCreateOptions::new(*az_css_downcast(css)); AzWindowCreateOptionsPtr { ptr: Box::into_raw(Box::new(object)) as *mut c_void } }
/// Destructor: Takes ownership of the `WindowCreateOptions` pointer and deletes it.
#[no_mangle] pub extern "C" fn az_window_create_options_delete(ptr: &mut AzWindowCreateOptionsPtr) { let _ = unsafe { Box::<WindowCreateOptions>::from_raw(ptr.ptr  as *mut WindowCreateOptions) }; }
/// Copies the pointer: WARNING: After calling this function you'll have two pointers to the same Box<`WindowCreateOptions`>!.
#[no_mangle] pub extern "C" fn az_window_create_options_shallow_copy(ptr: &AzWindowCreateOptionsPtr) -> AzWindowCreateOptionsPtr { AzWindowCreateOptionsPtr { ptr: ptr.ptr } }
/// (private): Downcasts the `AzWindowCreateOptionsPtr` to a `Box<WindowCreateOptions>`. Note that this takes ownership of the pointer.
fn az_window_create_options_downcast(ptr: AzWindowCreateOptionsPtr) -> Box<WindowCreateOptions> { unsafe { Box::<WindowCreateOptions>::from_raw(ptr.ptr  as *mut WindowCreateOptions) } }
