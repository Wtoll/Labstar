// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            tauri::async_runtime::set(tokio::runtime::Handle::current());

            tauri::Builder::default()
                .plugin(tauri_plugin_opener::init())
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        });
}
