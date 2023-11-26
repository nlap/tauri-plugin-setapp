use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

#[cfg(target_os = "macos")]
#[link(name = "SetappBridge", kind = "static")]
extern "C" {
    fn startPlugin();
    fn showReleaseNotesWindowIfNeeded();
    fn showReleaseNotesWindow();
    fn askUserToShareEmail();
}

#[tauri::command]
fn show_release_notes_window_if_needed() {
    cfg_if::cfg_if! {
      if #[cfg(target_os = "macos")] {
        unsafe { showReleaseNotesWindowIfNeeded() };
      }
    }
}

#[tauri::command]
fn show_release_notes_window() {
    cfg_if::cfg_if! {
      if #[cfg(target_os = "macos")] {
        unsafe { showReleaseNotesWindow() };
      }
    }
}

#[tauri::command]
fn ask_user_to_share_email() {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
          unsafe { askUserToShareEmail() };
      }
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            unsafe { startPlugin() };
            Builder::new("setapp")
                .invoke_handler(tauri::generate_handler![
                    show_release_notes_window_if_needed,
                    show_release_notes_window,
                    ask_user_to_share_email
                ])
                .build()
        } else {
            Builder::new("setapp").build()
        }
    }
}
