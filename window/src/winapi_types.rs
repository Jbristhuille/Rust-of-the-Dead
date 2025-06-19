/******************************************************************************
 * @Author                : Jbristhuille<jbristhuille@gmail.com>              *
 * @CreatedDate           : 2025-06-19 21:23:37                               *
 * @LastEditors           : Jbristhuille<jbristhuille@gmail.com>              *
 * @LastEditDate          : 2025-06-19 21:43:50                               *
 *****************************************************************************/

// window/src/winapi_types.rs

use std::ffi::c_void;

/// Basic WinAPI type aliases
pub type HINSTANCE = *mut c_void;
pub type HICON     = *mut c_void;
pub type HCURSOR   = *mut c_void;
pub type HBRUSH    = *mut c_void;
pub type LPCWSTR   = *const u16;
pub type HWND      = *mut c_void;
pub type DWORD     = u32;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;
pub type UINT = u32;

/// Pointer to a window procedure function (callback)
pub type WNDPROC = Option<
  unsafe extern "system" fn(HWND, u32, usize, isize) -> isize
>;

/// Structure passed to RegisterClassW
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

/// Common window styles
pub const WS_OVERLAPPEDWINDOW: DWORD = 0x00CF0000;
pub const CW_USEDEFAULT: i32 = 0x80000000_u32 as i32;

/// ShowWindow flags
pub const SW_SHOW: i32 = 5;
