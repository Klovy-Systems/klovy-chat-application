pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![badge::set_unread_badge])
        .on_page_load(|webview, payload| {
            badge::on_page_load(webview, payload.event());
        })
        .setup(|app| {
            discord_presence::init_from_app_config(app.config());
            updater::spawn(app.handle().clone());
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

mod badge;
mod discord_presence;
mod updater;
