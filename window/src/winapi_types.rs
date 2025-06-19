/******************************************************************************
 * @Author                : Jbristhuille<jbristhuille@gmail.com>              *
 * @CreatedDate           : 2025-06-19 21:06:52                               *
 * @LastEditors           : Jbristhuille<jbristhuille@gmail.com>              *
 * @LastEditDate          : 2025-06-19 21:20:14                               *
 *****************************************************************************/

use std::ffi::c_void;

/// WinAPI type aliases (simplified)
pub type HINSTANCE = *mut c_void;
pub type HICON     = *mut c_void;
pub type HCURSOR   = *mut c_void;
pub type HBRUSH    = *mut c_void;
pub type LPCWSTR   = *const u16;

/// Pointer to window procedure (to be refined later)
pub type WNDPROC = Option<unsafe extern "system" fn() -> isize>;

/// Minimal definition of WNDCLASSW for window registration
#[allow(non_snake_case)]
#[repr(C)]
pub struct WNDCLASSW {
  pub style: u32,
  pub lpfnWndProc: WNDPROC,
  pub cbClsExtra: i32,
  pub cbWndExtra: i32,
  pub hInstance: HINSTANCE,
  pub hIcon: HICON,
  pub hCursor: HCURSOR,
  pub hbrBackground: HBRUSH,
  pub lpszMenuName: LPCWSTR,
  pub lpszClassName: LPCWSTR,
}
