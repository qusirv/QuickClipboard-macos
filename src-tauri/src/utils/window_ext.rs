// 跨平台窗口 Builder 扩展
// Tauri 的 .transparent() / .drag_and_drop() 在 macOS 上不存在（WKWebView 限制），
// 这里提供统一的包装方法，macOS 上空转，其他平台调用原始方法。

use tauri::Manager;
use tauri::Runtime;
use tauri::WebviewWindowBuilder;

pub trait WindowBuilderExt<R: Runtime, M: Manager<R>> {
    fn transparent_cp(self, enabled: bool) -> Self;
    fn drag_and_drop_cp(self, enabled: bool) -> Self;
}

impl<R: Runtime, M: Manager<R>> WindowBuilderExt<R, M> for WebviewWindowBuilder<'_, R, M> {
    #[cfg(not(target_os = "macos"))]
    fn transparent_cp(self, enabled: bool) -> Self {
        self.transparent(enabled)
    }

    #[cfg(target_os = "macos")]
    fn transparent_cp(self, _enabled: bool) -> Self {
        self
    }

    #[cfg(not(target_os = "macos"))]
    fn drag_and_drop_cp(self, enabled: bool) -> Self {
        self.drag_and_drop(enabled)
    }

    #[cfg(target_os = "macos")]
    fn drag_and_drop_cp(self, _enabled: bool) -> Self {
        self
    }
}
