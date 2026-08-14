use std::{
    env, fs,
    path::{Path, PathBuf},
};

/// Read a file, from the CWD.
#[must_use]
pub fn read_from_file(file: &Path) -> String {
    let mut path = crate::get_cwd();
    path.push(file);

    let err = format!("Unable to read file {:?}", file);

    fs::read_to_string(path).expect(err.as_str())
}

/// Get the current active cwd.
#[must_use]
pub fn get_cwd() -> PathBuf {
    env::current_dir().expect("Unable to get cwd")
}
