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
    pub fn mrb_define_class(mrb: *mut mrb_state, name: *const c_char, super_class: *mut RClass) -> *mut RClass;
    pub fn mrb_define_class_under(mrb: *mut mrb_state, outer: *mut RClass, name: *const c_char, super_class: *mut RClass) -> *mut RClass;
    pub fn mrb_define_class_method(mrb: *mut mrb_state, klass: *mut RClass, name: *const c_char, function: mrb_func_t, spec: mrb_aspec);
    pub fn mrb_define_module_function(mrb: *mut mrb_state, klass: *mut RClass, name: *const c_char, function: mrb_func_t, spec: mrb_aspec);
    pub fn mrb_define_method(mrb: *mut mrb_state, klass: *mut RClass, name: *const c_char, function: mrb_func_t, spec: mrb_aspec);
    pub fn mrb_get_args(mrb: *mut mrb_state, format: mrb_args_format, ...) -> mrb_int;
    #[link_name = "tmrb_obj_value"]
    pub fn mrb_obj_value(p: *mut libc::c_void) -> mrb_value;

    // Array
    pub fn mrb_ary_ref(mrb: *mut mrb_state, array: mrb_value, length: mrb_int) -> mrb_value;
    #[link_name = "TRARRAY_LEN"]
    pub fn RARRAY_LEN(array: mrb_value) -> mrb_int;
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

    // Class
    #[link_name = "TMRB_SET_INSTANCE_TT"]
    pub fn MRB_SET_INSTANCE_TT(class: *mut RClass, tt: mrb_vtype);

    // mrb_state accessor
    #[link_name = "tmrb_state_object_class"]
    pub fn mrb_state_object_class(mrb: *mut mrb_state) -> *mut RClass;

    #[link_name = "tmrb_data_init"]
    pub fn mrb_data_init(v: mrb_value, ptr: *mut c_void, mrb_type: *const mrb_data_type);
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

// Data
pub type RData = c_void;

#[allow(non_camel_case_types)]
pub type data_free_func = extern "C" fn(mrb: *mut mrb_state, map: *mut c_void);

#[repr(C)]
pub struct mrb_data_type {
    pub dtype: *const c_char,
    pub dfree: data_free_func,
}

unsafe impl Sync for mrb_data_type {}

// Value
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum mrb_vtype {
    MRB_TT_FALSE = 0,   /*   0 */
    MRB_TT_FREE,        /*   1 */
    MRB_TT_TRUE,        /*   2 */
    MRB_TT_FIXNUM,      /*   3 */
    MRB_TT_SYMBOL,      /*   4 */
    MRB_TT_UNDEF,       /*   5 */
    MRB_TT_FLOAT,       /*   6 */
    MRB_TT_CPTR,        /*   7 */
    MRB_TT_OBJECT,      /*   8 */
    MRB_TT_CLASS,       /*   9 */
    MRB_TT_MODULE,      /*  10 */
    MRB_TT_ICLASS,      /*  11 */
    MRB_TT_SCLASS,      /*  12 */
    MRB_TT_PROC,        /*  13 */
    MRB_TT_ARRAY,       /*  14 */
    MRB_TT_HASH,        /*  15 */
    MRB_TT_STRING,      /*  16 */
    MRB_TT_RANGE,       /*  17 */
    MRB_TT_EXCEPTION,   /*  18 */
    MRB_TT_FILE,        /*  19 */
    MRB_TT_ENV,         /*  20 */
    MRB_TT_DATA,        /*  21 */
    MRB_TT_FIBER,       /*  22 */
    MRB_TT_MAXDEFINE    /*  23 */
}
