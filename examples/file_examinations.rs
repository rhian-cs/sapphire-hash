use std::path::Path;

/// Simple script that analyzes how Rust interprets the attributes of each file or directory
fn main() {
    let paths = vec![
        "/dev/urandom",
        "/inexistent",
        "tmp/random",
        "tmp/random/file_1",
    ];

    for path in paths {
        examine_path(path)
    }
}

fn examine_path(path_string: &str) {
    let path = Path::new(&path_string);

    println!("--- Examining path {:?} ---", path);
    println!("exists(): {}", path.exists());
    println!("is_absolute(): {}", path.is_absolute());
    println!("is_relative(): {}", path.is_relative());
    println!("is_dir(): {}", path.is_dir());
    println!("is_file(): {}", path.is_file());
    println!("is_symlink(): {}", path.is_symlink());
    println!("");
}
