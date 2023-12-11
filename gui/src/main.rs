use rfd::FileDialog;

slint::include_modules!();
fn main() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;

    window.on_input_path_button_clicked({
        let window = window.as_weak().unwrap();
        move || window.set_input_path(open_directory_input().into())
    });
    window.on_output_path_button_clicked({
        let window = window.as_weak().unwrap();
        move || window.set_output_path(open_directory_input().into())
    });

    window.on_calculate_button_clicked({
        let window = window.as_weak().unwrap();

        move || {
            let input_path = window.get_input_path();
            let output_path = window.get_output_path();
            window.set_error_message("".into());

            if input_path.is_empty() {
                window.set_error_message("Please select an input directory.".into());
                return;
            }

            if output_path.is_empty() {
                window.set_error_message("Please select a directory for the report to be saved.".into());
                return;
            }
        }
    });

    window.run()
}

fn open_directory_input() -> String {
    let directory = FileDialog::new().pick_folder();

    match directory {
        Some(path) => path.to_str().unwrap_or_default().to_owned(),
        None => String::default(),
    }
}
