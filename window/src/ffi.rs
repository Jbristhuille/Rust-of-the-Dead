/******************************************************************************
 * @Author                : Jbristhuille<jbristhuille@gmail.com>              *
 * @CreatedDate           : 2025-06-19 21:09:37                               *
 * @LastEditors           : Jbristhuille<jbristhuille@gmail.com>              *
 * @LastEditDate          : 2025-06-19 21:20:19                               *
 *****************************************************************************/

use crate::winapi_types::WNDCLASSW;

#[link(name = "user32")]
unsafe extern "system" {
  /// Registers a window class for subsequent use in calls to CreateWindow or CreateWindowEx.
  pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> u16;
}
