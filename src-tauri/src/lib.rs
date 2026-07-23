pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            discord_presence::init_from_app_config(app.config());
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_, event| {
            if let tauri::RunEvent::Exit = event {
                discord_presence::stop();
            }
        });
}

mod discord_presence;
