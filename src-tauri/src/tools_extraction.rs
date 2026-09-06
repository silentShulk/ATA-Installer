use std::fs::{create_dir_all, File};
use std::io::Write;
use tauri::State;

use crate::paths::Paths;

#[cfg(target_os = "linux")]
const ATA: &[u8] = include_bytes!("../embedded/ATA-x86_64-unknown-linux-gnu");
#[cfg(target_os = "windows")]
const ATA: &[u8] = include_bytes!("../embedded/ATA-x86_64-pc-windows-msvc.exe");

#[cfg(target_os = "linux")]
const SHELLUI: &[u8] = include_bytes!("../embedded/shellUI-x86_64-unknown-linux-gnu");
#[cfg(target_os = "windows")]
const SHELLUI: &[u8] = include_bytes!("../embedded/shellUI-x86_64-pc-windows-msvc.exe");

const SHELLUI_MANIFEST: &str = "name = \"ShellUI\"\nkind = \"App\"\n";

#[tauri::command]
pub fn extract_tools(paths: State<Paths>) -> Result<(), String> {
    extract_tools_inner(&paths).map_err(|er| er.to_string())
}

fn extract_tools_inner(paths: &Paths) -> Result<(), std::io::Error> {
    let mut ata = File::create(&paths.executable)?;
    ata.write_all(ATA)?;

    let shellui_folder = paths.apps_dir.join("ShellUI");
    create_dir_all(&shellui_folder)?;

    let mut shellui = File::create(shellui_folder.join("ShellUI"))?;
    shellui.write_all(SHELLUI)?;

    let mut manifest = File::create(shellui_folder.join("manifest.toml"))?;
    manifest.write_all(SHELLUI_MANIFEST.as_bytes())?;

    #[cfg(unix)] {
        use std::fs::{set_permissions, Permissions};
        use std::os::unix::fs::PermissionsExt;
        set_permissions(&paths.executable, Permissions::from_mode(0o755))?;
        set_permissions(shellui_folder.join("ShellUI"), Permissions::from_mode(0o755))?;
    }

    Ok(())
}