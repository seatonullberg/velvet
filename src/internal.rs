use std::env::current_exe;
use std::path::PathBuf;

// Returns the absolute path to a file in the `resources/` directory.
#[allow(dead_code)]
pub fn get_resource_filepath<'a>(filename: impl Into<&'a str>) -> PathBuf {
    let filename = filename.into();
    let mut path = current_exe().unwrap();
    path.pop();
    path.pop();
    path.pop();
    path.pop();
    path.push("resources");
    path.push(filename);
    path
}
