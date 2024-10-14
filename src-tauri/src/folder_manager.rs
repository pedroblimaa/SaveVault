use std::fs;

pub fn move_folder_items(from_path: &str, to_path: &str) {
    let entries = fs::read_dir(from_path).expect("Failed to read directory");

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        let file_name = path.file_name().expect("Failed to get file name").to_string_lossy();
        let new_path = format!("{}/{}", to_path, file_name);
        fs::rename(path, new_path).expect("Failed to move file");
    }
}
