use crate::{db, files_manager};

pub fn set_cloud_location(path: &str) {
    let cloud_location = db::db_manager::select_cloud_location();

    if !cloud_location.is_empty() && cloud_location != path {
        files_manager::move_folder_items(&cloud_location, path);
    }

    db::db_manager::set_cloud_folder(path);
}
