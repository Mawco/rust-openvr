use std::ffi::CString;
use std::os::raw::c_char;

use Settings;

impl Settings {
  pub fn get_bool(&self, a: String, b: String, c: *mut i32) -> bool {
    let _a = CString::new(a).expect("CString::new failed");
    let _b = CString::new(b).expect("CString::new failed");

    unsafe { self.0.GetBool.unwrap()(_a.as_ptr() as *mut i8, _b.as_ptr() as *mut i8, c) }.into()
  }
  
  pub fn get_float(&self, a: String, b: String, c: *mut i32) -> f32 {
    let _a = CString::new(a).expect("CString::new failed");
    let _b = CString::new(b).expect("CString::new failed");

    unsafe { self.0.GetFloat.unwrap()(_a.as_ptr() as *mut i8, _b.as_ptr() as *mut i8, c) }.into()
  }

  pub fn get_int32(&self, a: String, b: String, c: *mut i32) -> i32 {
    let _a = CString::new(a).expect("CString::new failed");
    let _b = CString::new(b).expect("CString::new failed");

    unsafe { self.0.GetInt32.unwrap()(_a.as_ptr() as *mut i8, _b.as_ptr() as *mut i8, c) }.into()
  }

  pub fn get_string(&self, section: &str, key: &str, buf: &mut [c_char], result: *mut i32) {
    let cstring_section = CString::new(section).expect("CString::new failed");
    let cstring_key = CString::new(key).expect("CString::new failed");

    unsafe {
        self.0.GetString.unwrap()(
          cstring_section.as_ptr() as *mut i8,
          cstring_key.as_ptr() as *mut i8,
          buf.as_mut_ptr() as *mut i8,
          buf.len() as u32,
          result,
        );
    }
}
}