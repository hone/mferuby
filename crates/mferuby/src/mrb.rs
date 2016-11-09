use std::mem;

extern crate libmruby_sys as sys;
extern crate libc;

pub struct Mrb {
    mrb: *mut sys::mrb_state
}

impl Mrb {
    pub fn new(mrb_arg: *mut sys::mrb_state) -> Mrb {
        Mrb { mrb: mrb_arg }
    }

    pub fn data_object_alloc<T>(&self, klass: sys::mrb_value, data: Box<T>, mrb_type: &sys::mrb_data_type) -> *mut sys::RData {
        unsafe {
            sys::mrb_data_object_alloc(self.mrb,
                                       mem::transmute(&klass),
                                       mem::transmute(data),
                                       mrb_type)
        }
    }
}
