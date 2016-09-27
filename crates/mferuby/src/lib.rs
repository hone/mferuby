extern crate libc;

use std::ffi::CString;
use libc::{c_void, c_char, c_int, c_double};
use std::thread;

macro_rules! cstr {
    ( $slice:expr ) => {
        CString::new($slice).unwrap().as_ptr()
    }
}

extern {
    #[link_name = "tmrb_nil_value"]
    pub fn nil() -> mrb_value;
    #[link_name = "tmrb_fixnum_value"]
    pub fn fixnum(i: c_int) -> mrb_value;
    #[link_name = "tmrb_float_value"]
    pub fn float(mrb: *mut mrb_state, f: c_double) -> mrb_value;

    pub fn mrb_open() -> *mut mrb_state;
    pub fn mrb_str_new_cstr(mrb: *mut mrb_state, c_str: *const c_char) -> mrb_value;
    pub fn mrb_define_module(mrb: *mut mrb_state, name: *const c_char) -> *mut RClass;
    pub fn mrb_define_class_method(mrb: *mut mrb_state, klass: *mut RClass, name: *const c_char, function: *const mrb_func_t, spec: mrb_aspec);

    #[link_name = "TMRB_ARGS_REQ"]
    pub fn MRB_ARGS_REQ(count: u32) -> mrb_aspec;
    #[link_name = "TMRB_ARGS_NONE"]
    pub fn MRB_ARGS_NONE() -> mrb_aspec;
}

#[allow(non_camel_case_types)]
pub type mrb_aspec = u32;

#[repr(C)]
pub struct mrb_state {
    padding: [u8; 376],
}

#[repr(C)]
pub struct mrb_value {
    padding: [usize; 2],
}

// these types are only referenced as pointers,
// so don't need to define opaque structs
pub type RClass = c_void;
pub type mrb_func_t = c_void;
