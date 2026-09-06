use std::{
    fs::{create_dir_all, read_dir, read_to_string, rename, File},
    path::PathBuf,
    process::Command,
};

use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Manager, State};
use thiserror::Error;

use serde_json::{from_str, to_writer_pretty, Value};

use crate::paths::Paths;

#[derive(Error, Debug)]
pub enum StyleError {
    #[error("IO operation failed. {0}")]
    Io(#[from] std::io::Error),
    #[error("Couldn't parse settings file. {0}")]
    Json(#[from] serde_json::Error),
    #[error("'{0}' is a parentless file, it is either the root folder or an empty string")]
    ParentlessFile(PathBuf),
    #[error("Couldn't read manifest file, it might be corrupted or not formatted correctly")]
    Toml(#[from] toml::de::Error),
}

/// Type of GUI, and where it lives on disk
#[derive(Clone, Serialize, Deserialize, Type, Debug, PartialEq)]
pub enum AppType {
    Webapp,
    App,
}
impl AppType {
    /// Base folder for this type of GUI (`uis_dir` or `apps_dir`), before joining the GUI's own name
    pub fn get_folder<'a>(&self, paths: &'a Paths) -> &'a PathBuf {
        match self {
            AppType::Webapp => &paths.uis_dir,
            AppType::App => &paths.apps_dir,
        }
    }
}

/// Everything the frontend needs to display and launch a GUI.
/// Built fresh on every scan — `executable`/`icon` are already resolved to absolute paths,
/// so nothing downstream needs to re-derive them from `name` + `kind`.
#[derive(Clone, Serialize, Deserialize, Type, Debug)]
pub struct Gui {
    pub name: String,
    pub kind: AppType,
    pub icon: Option<PathBuf>,
    pub executable: PathBuf,
}

/// On-disk manifest.toml shape
#[derive(Deserialize)]
struct AppManifest {
    name: String,
    kind: AppType,
    /// Filename of an icon image, relative to the manifest's own folder
    icon: Option<String>,
}

#[tauri::command]
#[specta::specta]
pub fn get_guis(paths: State<Paths>) -> Result<(Option<Gui>, Vec<Gui>), String> {
    get_guis_inner(&paths).map_err(|e| e.to_string())
}
fn get_guis_inner(paths: &Paths) -> Result<(Option<Gui>, Vec<Gui>), StyleError> {
    let mut guis = scan_for_guis(paths)?;
    let selected_name = get_selected_style(paths)?;

    let selected = guis
        .iter()
        .position(|g| g.name == selected_name)
        .map(|i| guis.remove(i));

    Ok((selected, guis))
}

fn scan_for_guis(paths: &Paths) -> Result<Vec<Gui>, StyleError> {
    let mut guis = Vec::new();
    scan_dir_for_guis(&paths.uis_dir, &mut guis)?;
    scan_dir_for_guis(&paths.apps_dir, &mut guis)?;
    Ok(guis)
}

/// Reads every subfolder of `dir` as a GUI folder. A folder with no/broken `manifest.toml`
/// is skipped and logged rather than failing the whole scan — one corrupt install
/// shouldn't hide every other GUI from the list.
fn scan_dir_for_guis(dir: &PathBuf, guis: &mut Vec<Gui>) -> Result<(), StyleError> {
    for entry in read_dir(dir)? {
        let gui_folder = entry?.path();
        if !gui_folder.is_dir() {
            continue;
        }

        let contents = match read_to_string(gui_folder.join("manifest.toml")) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Skipping {gui_folder:?}: no readable manifest.toml");
                continue;
            }
        };
        let manifest: AppManifest = match toml::from_str(&contents) {
            Ok(m) => m,
            Err(er) => {
                eprintln!("Skipping {gui_folder:?}: bad manifest.toml. {er}");
                continue;
            }
        };

        guis.push(Gui {
            // exe was normalized to a bare `name` (no extension) at install time,
            // see add_gui_inner
            executable: gui_folder.join(&manifest.name),
            icon: manifest.icon.map(|f| gui_folder.join(f)),
            name: manifest.name,
            kind: manifest.kind,
        });
    }
    Ok(())
}

fn get_selected_style(paths: &Paths) -> Result<String, StyleError> {
    let contents = read_to_string(&paths.settings_file)?;
    let settings: Value = from_str(&contents)?;
    Ok(settings["style"].as_str().unwrap_or("").to_string())
}

#[tauri::command]
pub fn set_selected_style(selected_style: String, paths: State<Paths>) -> Result<(), String> {
    set_selected_style_inner(selected_style, &paths).map_err(|er| er.to_string())
}
fn set_selected_style_inner(selected_style: String, paths: &Paths) -> Result<(), StyleError> {
    let contents = read_to_string(&paths.settings_file)?;
    let mut settings: Value = from_str(&contents)?;

    settings["style"] = Value::String(selected_style);

    let settings_file = File::create(&paths.settings_file)?;
    to_writer_pretty(settings_file, &settings)?;

    Ok(())
}

/// Installs a GUI: reads the manifest sitting next to the picked exe, then moves
/// EVERY file from that source folder into `<type_dir>/<name>/`. The exe itself is
/// renamed to a bare `name` (no extension) — extensions vary too much across
/// platforms/formats (.exe, .AppImage, none, .sh...) to rely on for launching, and a
/// direct full-path `Command::new(...)` spawn doesn't need one anyway. Every other
/// file (manifest.toml, icon, ...) keeps its own original filename.
#[tauri::command]
pub fn add_gui(path_to_new_gui: PathBuf, paths: State<Paths>) -> Result<(), String> {
    add_gui_inner(path_to_new_gui, &paths).map_err(|er| er.to_string())
}
fn add_gui_inner(path_to_new_gui: PathBuf, paths: &Paths) -> Result<(), StyleError> {
    let source_folder = path_to_new_gui
        .parent()
        .ok_or_else(|| StyleError::ParentlessFile(path_to_new_gui.clone()))?
        .to_path_buf();

    let contents = read_to_string(source_folder.join("manifest.toml"))?;
    let manifest: AppManifest = toml::from_str(&contents)?;

    let target_folder = manifest.kind.get_folder(paths).join(&manifest.name);
    create_dir_all(&target_folder)?;

    rename(&path_to_new_gui, target_folder.join(&manifest.name))?;

    for entry in read_dir(&source_folder)? {
        let entry = entry?;
        if entry.path() == path_to_new_gui {
            continue; // exe already handled above, with a renamed target
        }
        rename(entry.path(), target_folder.join(entry.file_name()))?;
    }

    Ok(())
}

/// `path_inside_gui_folder` only needs to be ANY file inside the GUI's folder
/// (in practice, the exe picked via the file dialog) — its parent folder is moved.
#[tauri::command]
pub fn remove_gui(path_inside_gui_folder: PathBuf, paths: State<Paths>) -> Result<(), String> {
    remove_gui_inner(path_inside_gui_folder, &paths).map_err(|er| er.to_string())
}
fn remove_gui_inner(path_inside_gui_folder: PathBuf, paths: &Paths) -> Result<(), StyleError> {
    let gui_folder = path_inside_gui_folder
        .parent()
        .ok_or_else(|| StyleError::ParentlessFile(path_inside_gui_folder.clone()))?;
    let folder_name = gui_folder
        .file_name()
        .ok_or_else(|| StyleError::ParentlessFile(gui_folder.to_path_buf()))?;

    rename(gui_folder, paths.downloads.join(folder_name)).map_err(StyleError::Io)
}

/// Hides the main window, runs the GUI's executable with its own folder as cwd,
/// then waits (off the main thread) for it to exit before showing the launcher again.
#[tauri::command]
pub fn launch_gui(app: AppHandle, gui: Gui) -> Result<(), String> {
    launch_gui_inner(app, gui).map_err(|er| er.to_string())
}
fn launch_gui_inner(app: AppHandle, gui: Gui) -> Result<(), StyleError> {
    let working_dir = gui.executable.parent().unwrap_or_else(|| std::path::Path::new("."));

    let mut child = Command::new(&gui.executable)
        .current_dir(working_dir)
        .spawn()?;

    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }

    std::thread::spawn(move || {
        let _ = child.wait();

        if let Some(window) = app.get_webview_window("main") {
            let _ = window.show();
            let _ = window.set_focus();
        }
    });

    Ok(())
}