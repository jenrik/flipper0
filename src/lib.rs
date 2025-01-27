#![cfg_attr(not(test), no_std)]
#![feature(custom_inner_attributes)]
#![cfg_attr(feature = "allocator", feature(alloc_error_handler))]
#![cfg_attr(feature = "allocator", feature(allocator_api))]
#![cfg_attr(feature = "allocator", feature(alloc_layout_extra))]
#![feature(core_intrinsics)]
#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]
#![feature(error_in_core)]
#![feature(const_trait_impl)]

#[macro_use]
extern crate alloc as _;
pub mod alloc;


// re-export flipper0-macro:
#[cfg_attr(feature = "macro", allow(unused_imports))]
#[cfg_attr(feature = "macro", macro_use)]
#[cfg(feature = "macro")]
extern crate proc_macros;
#[cfg(feature = "macro")]
pub use proc_macros::*;


pub mod ffi {
	//! Automatically generated low-level bindings.
	//!
	#![doc = core::env!("BINDINGS_METADATA_DOC", "Bindings metadata not found.")]
	#![allow(non_upper_case_globals)]
	#![allow(non_camel_case_types)]
	#![allow(non_snake_case)]
	#![allow(clippy::missing_safety_doc)]
	#![allow(clippy::useless_transmute)]
	#![allow(clippy::transmute_int_to_bool)]

	#[cfg(all(test, no_std))]
	use core::prelude::*;
	#[cfg(all(test, no_std))]
	use core::prelude::rust_2021::*;
	#[cfg(all(test, no_std))]
	use core::{assert_eq, assert};
	#[cfg(all(test, no_std))]
	use core::{debug_assert_eq, debug_assert};

	core::include!(core::env!("BINDINGS", "Bindings not found. Build-script failed."));


	#[no_mangle]
	unsafe fn __exidx_start() -> ! { panic!("__exidx_start") }
	#[no_mangle]
	unsafe fn __exidx_end() -> ! { panic!("__exidx_end") }
	#[no_mangle]
	unsafe fn __cxa_call_unexpected() -> ! { panic!("__cxa_call_unexpected") }
	#[no_mangle]
	unsafe fn __gnu_Unwind_Find_exidx() -> ! { panic!("__gnu_Unwind_Find_exidx") }
	#[no_mangle]
	unsafe fn __cxa_begin_cleanup() -> ! { panic!("__cxa_begin_cleanup") }
	#[no_mangle]
	unsafe fn __cxa_type_match() -> ! { panic!("__cxa_type_match") }
}


pub mod r#panic;
pub mod process;
pub mod error;
pub mod result;
pub mod os;
