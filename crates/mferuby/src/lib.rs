pub extern crate libmruby_sys as sys;

pub extern crate libc;

pub mod mrb;
pub use mrb::Mrb;

#[macro_export]
macro_rules! cstr {
    ( $slice:expr ) => {
        CString::new($slice).unwrap().as_ptr()
    }
}

pub fn mruby_str_to_rust_string(mruby_string: sys::mrb_value) -> Result<String, std::str::Utf8Error> {
    let size = unsafe { sys::RSTRING_LEN(mruby_string) };
    let ptr = unsafe { sys::RSTRING_PTR(mruby_string) };
    let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, size as usize) };
    let s = try!(std::str::from_utf8(slice));
    Ok(s.to_string())
}
