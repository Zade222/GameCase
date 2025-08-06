use std::path::{Path, PathBuf};
use std::fs::{self};

use crate::cli_error_handling::CliError;

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

/// Sorts input paths into separate vectors of files and directories.
///
/// This function iterates over a slice of `PathBuf`s and categorizes
/// each path based on whether it points to a file or a directory. Paths
/// that do not exist or are not files or directories are ignored.
///
/// # Arguments
///
/// * `input_paths`: A slice of `PathBuf` objects to be organized.
///
/// # Returns
///
/// A `Result` containing a tuple with two `Vec<PathBuf>`:
/// - The first vector contains all valid paths to files.
/// - The second vector contains all valid paths to directories.
pub fn organize_paths(
    input_paths: &[PathBuf],
) -> Result<(Vec<(String, PathBuf)>, Vec<(String, PathBuf)>), CliError> {
    //Create vector to store file paths.
    let mut file_paths: Vec<(String, PathBuf)> = Vec::new();

    //Create vector to store directory paths.
    let mut dir_paths: Vec<(String, PathBuf)> = Vec::new();

    /*For each path in the provided input_paths variable:
    - If path is a directory, add that path to the directory paths vector.
    - If path is a file, add that path to the file paths vector.*/
    for path in input_paths {
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        if path.is_dir() {
            dir_paths.push((format!("{file_name}/"), path.to_path_buf()));
        } else if path.is_file() {
            file_paths.push((format!("{file_name}/"), path.to_path_buf()));
        } else {
            // println!("WARN: Ignoring unsupported path type: {}", path.display());
        }
    }

    /*Return tuple containing the file and directory path vectors. */
    Ok((file_paths, dir_paths))
}