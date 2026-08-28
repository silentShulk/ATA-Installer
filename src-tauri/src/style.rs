use std::{fs::{File, read_dir, read_to_string, rename}, path::PathBuf};

use serde::Deserialize;
use tauri::State;
use thiserror::Error;

use serde_json::{from_str, to_writer_pretty, Value};

use crate::{paths::Paths};

#[derive(Error, Debug)]
pub enum StyleError {
    #[error("IO operation failed. {0}")]
    Io(#[from] std::io::Error),
    #[error("Couldn't parse settings file. {0}")]
    Json(#[from] serde_json::Error),
    #[error("'{0}' is a parentless file, it is either the root folder or an empty string")]
    ParentlessFile(PathBuf),
    #[error("Couldn't read manifest file, it might be corrupted or not formatted correctly")]
    Toml(#[from] toml::de::Error)
}


#[tauri::command]
pub fn get_styles(paths: State<Paths>) -> Result<(String, Vec<String>), String> {
    get_styles_inner(&paths).map_err(|e| e.to_string())
}
fn get_styles_inner(paths: &Paths) -> Result<(String, Vec<String>), StyleError> {
    let mut styles = scan_for_styles(&paths)?;
    let selected_style = get_selected_style(paths)?;
    
    styles.retain(|style| style != &selected_style);
    
    Ok((selected_style, styles))
}

fn scan_for_styles(paths: &Paths) -> Result<Vec<String>, StyleError> {
    let mut styles: Vec<String> = Vec::new();

    for entry in std::fs::read_dir(&paths.uis_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(stem) = path.file_stem() {
                let s = stem.to_string_lossy().into_owned();
                styles.push(s);
            }
        }
    }

    for entry in read_dir(&paths.apps_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(stem) = path.file_stem() {
                let s = stem.to_string_lossy().into_owned();
                styles.push(s);
            }
        }
    }

    Ok(styles)
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

#[tauri::command]
pub fn add_style(path_to_new_style: PathBuf, paths: State<Paths>) -> Result<(), String> {
    add_style_inner(path_to_new_style, &paths).map_err(|er| er.to_string())
}
fn add_style_inner(path_to_new_style: PathBuf, paths: &Paths) -> Result<(), StyleError> {
    let style_folder = path_to_new_style.parent()
        .ok_or_else(|| StyleError::ParentlessFile(path_to_new_style.clone()))?;
    let manifest_file_path = style_folder.join("manifest.toml");

    let contents = read_to_string(manifest_file_path)?;
    let manifest: AppManifest = toml::from_str(&contents)?;

    let exe_type_target_folder = manifest.get_folder_by_type(paths);
    rename(&path_to_new_style, exe_type_target_folder.join(manifest.name)).map_err(|er| {
        StyleError::Io(er)
    })
}

#[tauri::command]
pub fn remove_style(path_to_style_to_remove: PathBuf, paths: State<Paths>) -> Result<(), String> {
    remove_style_inner(path_to_style_to_remove, &paths).map_err(|er| er.to_string()) 
}
fn remove_style_inner(path_to_style_to_remove: PathBuf, paths: &Paths) -> Result<(), StyleError> {
    let Some(filename) = path_to_style_to_remove.file_name() else {
        return Err(StyleError::ParentlessFile(path_to_style_to_remove))
    };
    rename(&path_to_style_to_remove, paths.downloads.join(filename))
        .map_err(StyleError::Io)
}

#[derive(Deserialize)]
struct AppManifest {
    name: String,
    kind: AppType
}
impl AppManifest {
    fn get_folder_by_type<'a>(&self, paths: &'a Paths) -> &'a PathBuf {
        match self.kind {
            AppType::Webapp => &paths.uis_dir,
            AppType::App => &paths.apps_dir,
        }
    }
}
#[derive(Deserialize, Debug)]
enum AppType {
    Webapp,
    App,
}