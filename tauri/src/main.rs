// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;

static PORT: u16 = 3344;

fn launch_cli() {
    std::thread::spawn(|| {
        let args = vec!["bolt".to_string(), "--tauri".to_string()];

        lib_bolt::start(args, PORT);
    });
}

fn main() {
    launch_cli();

    let app = tauri::Builder::default().invoke_handler(tauri::generate_handler![]);

    app.run(tauri::generate_context!()).unwrap();
}
