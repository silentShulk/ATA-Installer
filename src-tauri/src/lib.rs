use tauri_specta::{collect_commands, Builder};
use specta_typescript::Typescript;

mod checks;
mod installation;
mod paths;
mod style;
mod tools_extraction;

use checks::check_installation_state;
use installation::{create_default_data, create_default_settings, create_folders};
use paths::{Paths, get_paths};
use style::{get_guis, set_selected_style, add_gui, remove_gui, launch_gui};
use tools_extraction::extract_tools;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta_builder = Builder::<tauri::Wry>::new()
        .commands(collect_commands![get_paths, get_guis]);

    #[cfg(debug_assertions)]
    specta_builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Paths::new())
        .invoke_handler(tauri::generate_handler![
            check_installation_state,
            get_guis,
            set_selected_style,
            create_folders,
            create_default_data,
            create_default_settings,
            get_paths,
            add_gui,
            remove_gui,
            launch_gui,
            extract_tools
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}