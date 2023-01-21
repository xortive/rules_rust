//! A Cargo build script binary used in unit tests for the Bazel `cargo_build_script` rule

use std::collections::HashSet;
use std::path::PathBuf;

fn main() {
    // The cargo_build_script macro appends an underscore to the given name.
    //
    // This file would be the only expected source file within the CARGO_MANIFEST_DIR without
    // any exec root symlink functionality.
    let build_script = PathBuf::from(
        std::env::args()
            .next()
            .expect("Unable to get the build script executable"),
    );

    let build_script_name = build_script
        .file_name()
        .expect("Unable to get the build script name")
        .to_str()
        .expect("Unable to convert the build script name to a string");

    let mut root_files = std::fs::read_dir(".")
        .expect("Unable to read the current directory")
        .map(|entry| {
            entry
                .expect("Failed to get entry")
                .file_name()
                .into_string()
                .expect("Failed to convert file name to string")
        })
        .collect::<HashSet<_>>();

    assert!(
        root_files.take(build_script_name).is_some(),
        "Build script must be in the current directory"
    );
    assert!(
        root_files.take("root_file.txt").is_some(),
        "'root_file.txt' must be in the current directory"
    );
    assert!(
        root_files.take("bazel-out").is_some(),
        "'bazel-out' must be in the current directory"
    );
    assert!(
        root_files.take("external").is_some(),
        "'external' must be in the current directory"
    );

    assert!(
        root_files.is_empty(),
        "There should not be any other files in the current directory, found {:?}",
        root_files
    );
}
