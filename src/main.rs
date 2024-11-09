// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::{SharedString, VecModel};
use std::error::Error;
use std::fs;

slint::include_modules!();

fn get_vscode_dirs() -> std::io::Result<Option<Vec<String>>> {
    let downloads = dirs::download_dir().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Downloads directory not found",
        )
    })?;

    let vscode_dirs: Vec<String> = fs::read_dir(downloads)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            let file_name = path.file_name()?.to_str()?;

            if path.is_dir() && file_name.starts_with("vscode-") {
                Some(
                    file_name
                        .to_string()
                        .strip_prefix("vscode-")
                        .unwrap_or(file_name)
                        .into(),
                )
            } else {
                None
            }
        })
        .collect();

    Ok(if vscode_dirs.is_empty() {
        None
    } else {
        Some(vscode_dirs)
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let vscodedirs = get_vscode_dirs()?;

    match vscodedirs {
        Some(dirs) => {
            let first_three = &dirs[..3.min(dirs.len())];
            let _quicklinks: Vec<SharedString> =
                vec!["arduino".into(), "python".into(), "rust".into()];
            let boo: Vec<SharedString> = first_three.iter().map(SharedString::from).collect();
            let labels = VecModel::from(boo);
            let labels_model = slint::ModelRc::new(labels);
            ui.set_labels(labels_model);
        }
        None => {
            println!("no vscode found");
            return Err("no vscode found".into());
        }
    }

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    ui.run()?;

    Ok(())
}
