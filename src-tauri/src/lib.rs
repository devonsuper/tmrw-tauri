mod config;
use config::PERSON;

use tauri::path::BaseDirectory;
use tauri::Manager;

#[tauri::command]
fn get_person() -> String {
    PERSON.into()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Serve bundled files via custom scheme: "bundle://"
        .register_uri_scheme_protocol("bundle", |ctx, request| {
            use std::{fs::File, io::Read};

            // Request URI path, e.g. "/assets/video.mp4"
            let path = request.uri().path();
            let rel_path = path.trim_start_matches('/'); // "assets/video.mp4"

            // Resolve inside bundled resources: see `bundle.resources` in tauri.conf.json
            let app = ctx.app_handle();
            let resource_path = match app.path().resolve(rel_path, BaseDirectory::Resource) {
                Ok(p) => p,
                Err(_) => {
                    return tauri::http::Response::builder()
                        .status(404)
                        .body(Vec::<u8>::new())
                        .unwrap();
                }
            };

            let mut file = match File::open(resource_path) {
                Ok(f) => f,
                Err(_) => {
                    return tauri::http::Response::builder()
                        .status(404)
                        .body(Vec::<u8>::new())
                        .unwrap();
                }
            };

            let mut bytes = Vec::new();
            if let Err(_) = file.read_to_end(&mut bytes) {
                return tauri::http::Response::builder()
                    .status(500)
                    .body(Vec::<u8>::new())
                    .unwrap();
            }

            tauri::http::Response::builder()
                .status(200)
                .header(tauri::http::header::CONTENT_TYPE, "video/mp4")
                .body(bytes)
                .unwrap()
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_person])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// mod config;
// use config::PERSON;

// #[tauri::command]
// fn get_person() -> String {
//     return PERSON.into();
// }

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     tauri::Builder::default()

//         .plugin(tauri_plugin_opener::init())
//         .invoke_handler(tauri::generate_handler![get_person])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

// mod config;
// use config::PERSON;
// use tauri::{webview::WebviewWindowBuilder, WebviewUrl};

// #[tauri::command]
// fn get_person() -> String {
//     return PERSON.into();
// }

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     let port: u16 = 1420;

//     tauri::Builder::default()
//         .plugin(tauri_plugin_localhost::Builder::new(port).build())
//         .plugin(tauri_plugin_opener::init())
//         .invoke_handler(tauri::generate_handler![get_person])
//         .setup(move |app| {
//             let url = format!("http://localhost:{}", port).parse().unwrap();
//             WebviewWindowBuilder::new(app, "main".to_string(), WebviewUrl::External(url))
//                 .title("Localhost Example")
//                 .build()?;
//             Ok(())
//         })
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }