/******************************************************************************
 * @Author                : Jbristhuille<jbristhuille@gmail.com>              *
 * @CreatedDate           : 2025-06-19 21:14:35                               *
 * @LastEditors           : Jbristhuille<jbristhuille@gmail.com>              *
 * @LastEditDate          : 2025-06-19 21:15:57                               *
 *****************************************************************************/

use crate::winapi_types::*;
use crate::ffi::RegisterClassW;
use std::ffi::c_void;
use std::ptr;

pub fn create_window() {
  // UTF-16 null-terminated class name
  let class_name: Vec<u16> = "MyWindowClass\0".encode_utf16().collect();

  let wnd_class = WNDCLASSW {
    style: 0,
    lpfnWndProc: Some(dummy_wndproc), // temporary message handler
    cbClsExtra: 0,
    cbWndExtra: 0,
    hInstance: ptr::null_mut(),
    hIcon: ptr::null_mut(),
    hCursor: ptr::null_mut(),
    hbrBackground: ptr::null_mut(),
    lpszMenuName: ptr::null(),
    lpszClassName: class_name.as_ptr(),
  };

  unsafe {
    let result = RegisterClassW(&wnd_class);
    if result == 0 {
      panic!("RegisterClassW failed");
    } else {
      println!("Window class registered successfully.");
    }
  }
}

unsafe extern "system" fn dummy_wndproc() -> isize {
  0
}
