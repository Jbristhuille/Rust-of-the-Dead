/******************************************************************************
 * @Author                : Jbristhuille<jbristhuille@gmail.com>              *
 * @CreatedDate           : 2025-06-19 21:47:26                               *
 * @LastEditors           : Jbristhuille<jbristhuille@gmail.com>              *
 * @LastEditDate          : 2025-06-19 22:00:12                               *
 *****************************************************************************/

// window/src/ffi.rs

use crate::winapi_types::*;
use std::ffi::c_void;

#[link(name = "kernel32")]
unsafe extern "system" {
  pub fn GetLastError() -> u32;
  pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HINSTANCE;
}

#[link(name = "user32")]
unsafe extern "system" {
  pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> u16;
  pub fn CreateWindowExW(
    dwExStyle: u32,
    lpClassName: LPCWSTR,
    lpWindowName: LPCWSTR,
    dwStyle: u32,
    x: i32,
    y: i32,
    nWidth: i32,
    nHeight: i32,
    hWndParent: *mut c_void,
    hMenu: *mut c_void,
    hInstance: HINSTANCE,
    lpParam: *mut c_void,
  ) -> HWND;
  pub fn ShowWindow(hWnd: HWND, nCmdShow: i32) -> i32;
  pub fn UpdateWindow(hWnd: HWND) -> i32;
  pub fn DefWindowProcW(hWnd: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
  pub fn GetMessageW(lpMsg: *mut MSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> i32;
  pub fn TranslateMessage(lpMsg: *const MSG) -> i32;
  pub fn DispatchMessageW(lpMsg: *const MSG) -> isize;
  pub fn PostQuitMessage(exit_code: i32);
}
