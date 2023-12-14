use rfd::FileDialog;

pub fn open_directory_dialog() -> String {
    let directory = FileDialog::new().pick_folder();

    match directory {
        Some(path) => path.to_str().unwrap_or_default().to_owned(),
        None => String::default(),
    }
}
