extern crate libc;

use libc::{c_void, c_char, c_int, c_double};

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
    pub fn mrb_define_class_method(mrb: *mut mrb_state, klass: *mut RClass, name: *const c_char, function: mrb_func_t, spec: mrb_aspec);
    pub fn mrb_define_module_function(mrb: *mut mrb_state, klass: *mut RClass, name: *const c_char, function: mrb_func_t, spec: mrb_aspec);
    pub fn mrb_get_args(mrb: *mut mrb_state, format: mrb_args_format, ...) -> mrb_int;
    // Array
    pub fn mrb_ary_ref(mrb: *mut mrb_state, array: mrb_value, length: mrb_int) -> mrb_value;
    // String
    pub fn mrb_str_to_cstr(mrb: *mut mrb_state, mrb: mrb_value) -> *mut c_char;
    #[link_name = "TRSTRING_LEN"]
    pub fn RSTRING_LEN(string: mrb_value) -> mrb_int;
    #[link_name = "TRSTRING_PTR"]
    pub fn RSTRING_PTR(string: mrb_value) -> *const c_char;

    #[link_name = "TMRB_ARGS_REQ"]
    pub fn MRB_ARGS_REQ(count: u32) -> mrb_aspec;
    #[link_name = "TMRB_ARGS_NONE"]
    pub fn MRB_ARGS_NONE() -> mrb_aspec;
    // Array
    #[link_name = "TRARRAY_LEN"]
    pub fn RARRAY_LEN(array: mrb_value) -> mrb_int;
}

#[allow(non_camel_case_types)]
pub type mrb_aspec = u32;
#[allow(non_camel_case_types)]
pub type mrb_int = c_int;
#[allow(non_camel_case_types)]
pub type mrb_args_format = *const c_char;
// these types are only referenced as pointers,
// so don't need to define opaque structs
pub type RClass = c_void;
#[allow(non_camel_case_types)]
pub type mrb_func_t = *mut c_void;

#[repr(C)]
pub struct mrb_state {
    padding: [u8; 376],
}

#[repr(C)]
#[derive(Copy,Clone)]
pub struct mrb_value {
    padding: [usize; 2],
}

// Array
pub type RArray = c_void;
