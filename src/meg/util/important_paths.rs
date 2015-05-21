use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use util::{TurboResult, human, ChainError};

/// Iteratively search for `file` in `pwd` and its parents, returning
/// the path of the directory.
pub fn find_project(pwd: &Path, file: &str) -> TurboResult<PathBuf> {
    find_project_manifest(pwd, file).map(|mut p| {
        // remove the file, leaving just the directory
        p.pop();
        p
    })
}

/// Iteratively search for `file` in `pwd` and its parents, returning
/// the path to the file.
pub fn find_project_manifest(pwd: &Path, file: &str) -> TurboResult<PathBuf> {
    let mut current = pwd;

    loop {
        let manifest = current.join(file);
        if fs::metadata(&manifest).is_ok() {
            return Ok(manifest)
        }

        match current.parent() {
            Some(p) => current = p,
            None => break,
        }
    }

    Err(human(format!("Could not find `{}` in `{}` or any parent directory",
                      file, pwd.display())))
}
