/******************************************************************************
 * @Author                : Jbristhuille<jbristhuille@gmail.com>              *
 * @CreatedDate           : 2025-06-19 21:47:06                               *
 * @LastEditors           : Jbristhuille<jbristhuille@gmail.com>              *
 * @LastEditDate          : 2025-06-19 22:02:42                               *
 *****************************************************************************/

use crate::winapi_types::*;
use crate::ffi::{
  CreateWindowExW, DefWindowProcW, GetLastError, GetModuleHandleW,
  RegisterClassW, ShowWindow, UpdateWindow, GetMessageW, TranslateMessage,
  DispatchMessageW, PostQuitMessage
};
use std::ptr;

static CLASS_NAME: &[u16] = &[
  b'M' as u16, b'y' as u16, b'W' as u16, b'i' as u16, b'n' as u16,
  b'd' as u16, b'o' as u16, b'w' as u16, b'C' as u16, b'l' as u16,
  b'a' as u16, b's' as u16, b's' as u16, 0u16
];

static WINDOW_TITLE: &[u16] = &[
  b'R' as u16, b'u' as u16, b's' as u16, b't' as u16, b' ' as u16,
  b'W' as u16, b'i' as u16, b'n' as u16, b'd' as u16, b'o' as u16,
  b'w' as u16, 0u16
];

pub fn create_window() {
  let hinstance = unsafe { GetModuleHandleW(ptr::null()) };

  let wnd_class = WNDCLASSW {
    style: 0,
    lpfnWndProc: Some(dummy_wndproc),
    cbClsExtra: 0,
    cbWndExtra: 0,
    hInstance: hinstance,
    hIcon: ptr::null_mut(),
    hCursor: ptr::null_mut(),
    hbrBackground: (1 + 1) as HBRUSH, // COLOR_WINDOW + 1
    lpszMenuName: ptr::null(),
    lpszClassName: CLASS_NAME.as_ptr(),
  };

  unsafe {
    let result = RegisterClassW(&wnd_class);
    if result == 0 {
      panic!("RegisterClassW failed");
    }

    let hwnd = CreateWindowExW(
      0,
      CLASS_NAME.as_ptr(),
      WINDOW_TITLE.as_ptr(),
      WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE,
      CW_USEDEFAULT,
      CW_USEDEFAULT,
      800,
      600,
      ptr::null_mut(),
      ptr::null_mut(),
      hinstance,
      ptr::null_mut(),
    );

    if hwnd.is_null() {
      let code = GetLastError();
      panic!("CreateWindowExW failed with error code: {}", code);
    }

    ShowWindow(hwnd, SW_SHOW);
    UpdateWindow(hwnd);

    println!("Window successfully created and displayed.");
  }

  let mut msg = MSG {
    hwnd: ptr::null_mut(),
    message: 0,
    wParam: 0,
    lParam: 0,
    time: 0,
    pt: POINT { x: 0, y: 0 },
  };

  unsafe {
    while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {
      TranslateMessage(&msg);
      DispatchMessageW(&msg);
    }
  }
}

pub unsafe extern "system" fn dummy_wndproc(
  hwnd: HWND,
  msg: UINT,
  wparam: WPARAM,
  lparam: LPARAM,
) -> LRESULT {
  match msg {
    WM_KEYDOWN => {
      if wparam == VK_ESCAPE as usize {
        unsafe { PostQuitMessage(0) }
        return 0;
      }
    }
    WM_DESTROY => {
      unsafe { PostQuitMessage(0) }
      return 0;
    }
    _ => {}
  }

  unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

