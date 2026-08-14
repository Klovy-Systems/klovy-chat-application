use std::time::Duration;
use tauri::{AppHandle, Runtime};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use tauri_plugin_updater::UpdaterExt;

pub fn spawn<R: Runtime>(app: AppHandle<R>) {
    if cfg!(debug_assertions) {
        return;
    }
    if !endpoints_configured(app.config()) {
        return;
    }

    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(4));
        tauri::async_runtime::block_on(async move {
            if let Err(err) = check_and_install(app).await {
                eprintln!("updater: {err}");
            }
        });
    });
}

fn endpoints_configured(config: &tauri::Config) -> bool {
    config
        .plugins
        .0
        .get("updater")
        .and_then(|value| value.get("endpoints"))
        .and_then(|endpoints| endpoints.as_array())
        .is_some_and(|endpoints| !endpoints.is_empty())
}

async fn check_and_install<R: Runtime>(app: AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    let Some(update) = app.updater()?.check().await? else {
        return Ok(());
    };

    let (tx, rx) = std::sync::mpsc::channel();
    let dialog_app = app.clone();
    let version = update.version.clone();
    app.run_on_main_thread(move || {
        let install = dialog_app
            .dialog()
            .message(format!(
                "Dostępna jest nowa wersja {version}.\n\nZainstalować teraz? Aplikacja uruchomi się ponownie."
            ))
            .title("Aktualizacja Klovy Chat")
            .kind(MessageDialogKind::Info)
            .buttons(MessageDialogButtons::OkCancelCustom(
                "Zainstaluj".into(),
                "Później".into(),
            ))
            .blocking_show();
        let _ = tx.send(install);
    })?;

    if !rx.recv().unwrap_or(false) {
        return Ok(());
    }

    update
        .download_and_install(|_, _| {}, || {})
        .await?;

    app.restart();
}
