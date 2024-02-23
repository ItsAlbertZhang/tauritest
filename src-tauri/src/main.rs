// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    println!("hello Rust!");
    tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::Ready { .. } => {
                println!("app is ready");
            }
            tauri::RunEvent::ExitRequested { .. } => {
                println!("exiting app");
                // sleep 10s
                std::thread::sleep(std::time::Duration::from_secs(3));
                // print exit
                println!("bye bye!");
            }
            _ => {}
        });
}
