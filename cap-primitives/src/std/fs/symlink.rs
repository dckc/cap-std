//! This defines `symlink`, the primary entrypoint to sandboxed `symlink`.

use crate::fs::symlink_impl;
#[cfg(debug_assertions)]
use crate::fs::{get_path, stat_unchecked, FollowSymlinks};
use std::{fs, io, path::Path};

/// Perform a `symlinkat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
pub fn symlink(old_path: &Path, new_start: &fs::File, new_path: &Path) -> io::Result<()> {
    // Call `symlink`.
    let result = symlink_impl(old_path, new_start, new_path);

    // Do an unsandboxed lookup and check that we found the same result.
    // TODO: This is a racy check, though it is useful for testing and fuzzing.
    #[cfg(debug_assertions)]
    match stat_unchecked(new_start, new_path, FollowSymlinks::No) {
        Ok(metadata) => match &result {
            Ok(()) => debug_assert!(metadata.file_type().is_symlink()),
            Err(e) => match e.kind() {
                io::ErrorKind::AlreadyExists | io::ErrorKind::PermissionDenied => (),
                _ => panic!(
                    "unexpected error opening start='{:?}', path='{}': {:?}",
                    get_path(new_start),
                    new_path.display(),
                    e
                ),
            },
        },
        Err(unchecked_error) => match &result {
            Ok(()) => panic!(
                "unexpected success opening start='{:?}', path='{}'; expected {:?}",
                get_path(new_start),
                new_path.display(),
                unchecked_error
            ),
            Err(result_error) => match result_error.kind() {
                io::ErrorKind::PermissionDenied => (),
                _ => {
                    assert_eq!(result_error.to_string(), unchecked_error.to_string());
                    assert_eq!(result_error.kind(), unchecked_error.kind());
                }
            },
        },
    }

    result
}