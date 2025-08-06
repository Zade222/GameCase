use std::path::{Path, PathBuf};
use std::fs::{self};

/// Checks if a given path points to a regular file.
///
/// This function uses `fs::symlink_metadata` to get information about
/// the path without following symbolic links. This is useful for ensuring
/// that a path is a file and not a link pointing to one, which is
/// important when recursively scanning directories.
///
/// # Arguments
///
/// * `path`: A reference to a `Path` object to be checked.
///
/// # Returns
///
/// Returns `true` if the path exists and is a regular file. Returns
/// `false` otherwise, or if an error occurs.
fn is_regular_file(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .map(|m| m.file_type().is_file()) 
        .unwrap_or(false) 
}

pub fn list_dir_items(input_path: &Path) -> Vec<(String, PathBuf)> {
    let mut paths: Vec<(String, PathBuf)> = Vec::new();
    let mut directories: Vec<(String, PathBuf)> = Vec::new();
    let mut files: Vec<(String, PathBuf)> = Vec::new();

    if let Some(parent) = input_path.parent() {
        paths.push(("../".to_string(), parent.to_path_buf()));
    }

    if let Ok(dir_items) = fs::read_dir(input_path) {
        for entry in dir_items.flatten() {
            let path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();

            if path.is_dir() {
                directories.push((format!("{file_name}/"), path));
            } else if is_regular_file(&path) {
                files.push((file_name, path));
            }
        }
    }

    directories.sort_by(|a, b| a.0.cmp(&b.0));
    files.sort_by(|a, b| a.0.cmp(&b.0));

    paths.append(&mut directories);
    paths.append(&mut files);

    paths
}