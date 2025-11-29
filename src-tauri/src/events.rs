use once_cell::sync::OnceCell;
use tauri::{AppHandle, Emitter};

static APP_HANDLE: OnceCell<AppHandle> = OnceCell::new();

pub fn set_app_handle(handle: AppHandle) {
    APP_HANDLE.set(handle).ok();
}

pub fn emit_event<P>(name: &str, payload: P)
where
    P: serde::Serialize + Clone + 'static,
{
    if let Some(app) = APP_HANDLE.get() {
        app.emit(name, payload).expect("Failed to emit event");
    }
}
