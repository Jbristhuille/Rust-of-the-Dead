/******************************************************************************
 * @Author                : Jbristhuille<jbristhuille@gmail.com>              *
 * @CreatedDate           : 2025-06-19 21:23:37                               *
 * @LastEditors           : Jbristhuille<jbristhuille@gmail.com>              *
 * @LastEditDate          : 2025-06-19 21:59:59                               *
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
pub const CW_USEDEFAULT: i32 = 0x80000000_u32 as i32;
pub const WS_OVERLAPPED: DWORD   = 0x00000000;
pub const WS_CAPTION: DWORD      = 0x00C00000;
pub const WS_SYSMENU: DWORD      = 0x00080000;
pub const WS_MINIMIZEBOX: DWORD  = 0x00020000;
pub const WS_VISIBLE: DWORD      = 0x10000000;
pub const WM_KEYDOWN: UINT = 0x0100;
pub const WM_DESTROY: UINT = 0x0002;
pub const VK_ESCAPE: u32 = 0x1B;

/// ShowWindow flags
pub const SW_SHOW: i32 = 5;

#[repr(C)]
pub struct POINT {
  pub x: i32,
  pub y: i32,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct MSG {
  pub hwnd: HWND,
  pub message: UINT,
  pub wParam: WPARAM,
  pub lParam: LPARAM,
  pub time: DWORD,
  pub pt: POINT,
}
