use crate::db;

use super::files_service;

pub fn is_cloud_location_empty() -> bool {
    let cloud_location = db::db_manager::select_cloud_location();

    cloud_location.is_empty()
}

pub fn set_cloud_location(path: &str, move_files: bool) {
    let cloud_location = db::db_manager::select_cloud_location();

    if !cloud_location.is_empty() && move_files {
        files_service::move_folder_items(&cloud_location, path);
    }

    db::db_manager::set_cloud_folder(path);
}
