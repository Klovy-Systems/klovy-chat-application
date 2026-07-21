#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            discord_presence::init_from_app_config(app.config());
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_, event| {
            if let tauri::RunEvent::Exit = event {
                #[cfg(not(any(target_os = "android", target_os = "ios")))]
                discord_presence::stop();
            }
        });
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod discord_presence;
