use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// The expected extension of rustfmt manifest files generated by `rustfmt_aspect`.
pub const RUSTFMT_MANIFEST_EXTENSION: &str = "rustfmt";

/// A struct containing details used for executing rustfmt.
#[derive(Debug)]
pub struct RustfmtConfig {
    /// The rustfmt binary from the currently active toolchain
    pub rustfmt: PathBuf,

    /// The rustfmt config file containing rustfmt settings.
    /// https://rust-lang.github.io/rustfmt/
    pub config: PathBuf,
}

/// Parse command line arguments and environment variables to
/// produce config data for running rustfmt.
pub fn parse_rustfmt_config() -> RustfmtConfig {
    let runfiles = runfiles::Runfiles::create().unwrap();

    let rustfmt = runfiles.rlocation(format!(
        "{}/{}",
        runfiles.current_repository(),
        env!("RUSTFMT")
    ));
    if !rustfmt.exists() {
        panic!("rustfmt does not exist at: {}", rustfmt.display());
    }

    let config = runfiles.rlocation(format!(
        "{}/{}",
        runfiles.current_repository(),
        env!("RUSTFMT_CONFIG")
    ));
    if !config.exists() {
        panic!(
            "rustfmt config file does not exist at: {}",
            config.display()
        );
    }

    RustfmtConfig { rustfmt, config }
}

/// A struct of target specific information for use in running `rustfmt`.
#[derive(Debug)]
pub struct RustfmtManifest {
    /// The Rust edition of the Bazel target
    pub edition: String,

    /// A list of all (non-generated) source files for formatting.
    pub sources: Vec<PathBuf>,
}

/// Parse rustfmt flags from a manifest generated by builds using `rustfmt_aspect`.
pub fn parse_rustfmt_manifest(manifest: &Path) -> RustfmtManifest {
    let content = fs::read_to_string(manifest)
        .unwrap_or_else(|_| panic!("Failed to read rustfmt manifest: {}", manifest.display()));

    let mut lines: Vec<String> = content
        .split('\n')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect();

    let edition = lines
        .pop()
        .expect("There should always be at least 1 line in the manifest");
    edition
        .parse::<i32>()
        .expect("The edition should be a numeric value. eg `2018`.");

    let runfiles = runfiles::Runfiles::create().unwrap();

    RustfmtManifest {
        edition,
        sources: lines
            .into_iter()
            .map(|src| runfiles.rlocation(format!("{}/{}", runfiles.current_repository(), src)))
            .collect(),
    }
}

#[cfg(target_family = "windows")]
const PATH_ENV_SEP: &str = ";";

#[cfg(target_family = "unix")]
const PATH_ENV_SEP: &str = ":";

/// Parse the runfiles of the current executable for manifests generated
/// by the `rustfmt_aspect` aspect.
pub fn find_manifests() -> Vec<PathBuf> {
    let runfiles = runfiles::Runfiles::create().unwrap();

    std::env::var("RUSTFMT_MANIFESTS")
        .map(|var| {
            var.split(PATH_ENV_SEP)
                .filter_map(|path| match path.is_empty() {
                    true => None,
                    false => Some(runfiles.rlocation(format!(
                        "{}/{}",
                        runfiles.current_repository(),
                        path
                    ))),
                })
                .collect()
        })
        .unwrap_or_default()
}
