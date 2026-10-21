//! Builds the frontend packages of the server before the server itself is built.
//!
//! The frontend lives in its own workspace under `frontend/`: every page is
//! a package of that workspace, and every package is a WebAssembly
//! application of the euv framework. This build script walks the packages,
//! builds each of them with the euv command line interface and writes the
//! artifacts into the static resource directory the server serves.
//!
//! The build is skipped when the frontend workspace is absent, which is the
//! case for the published crate, and when `EUV_FRONTEND_SKIP` is set, which
//! is the case for the pipeline jobs that do not ship the browser assets.

mod frontend_build;

use std::{
    env::var,
    fs::{self, File, create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::SystemTime,
};

use crate::frontend_build::*;

/// Entry point of the build script.
///
/// Builds every page package of the frontend workspace and writes the
/// artifacts into the static resource directory.
///
/// # Returns
///
/// - `Result<(), Box<dyn std::error::Error>>` - The frontend is built, the
///   build was skipped, or the build failed.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir: PathBuf = PathBuf::from(var(ENV_MANIFEST_DIR)?);
    let frontend_dir: PathBuf = manifest_dir.join(FRONTEND_DIR);
    println!(
        "cargo:rerun-if-changed={}",
        frontend_dir.join(PAGES_DIR).display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        frontend_dir.join("Cargo.toml").display()
    );
    println!("cargo:rerun-if-env-changed={ENV_SKIP}");
    println!("cargo:rerun-if-env-changed={ENV_CLI}");
    if var(ENV_SKIP).is_ok() {
        println!("cargo:warning=the frontend build is skipped by {ENV_SKIP}");
        return Ok(());
    }
    if !frontend_dir.join(PAGES_DIR).is_dir() {
        println!("cargo:warning=the frontend workspace is absent, no page is built");
        return Ok(());
    }
    let release: bool = var(ENV_PROFILE).unwrap_or_default() == PROFILE_RELEASE;
    let packages: Vec<PathBuf> = page_packages(&frontend_dir);
    if packages.is_empty() {
        println!("cargo:warning=the frontend workspace holds no page package");
        return Ok(());
    }
    for package in packages {
        build_page(&manifest_dir, &package, release)?;
    }
    Ok(())
}

/// Collects the page packages of the frontend workspace.
///
/// # Arguments
///
/// - `&Path` - The frontend workspace.
///
/// # Returns
///
/// - `Vec<PathBuf>` - The package directory of every page.
fn page_packages(frontend_dir: &Path) -> Vec<PathBuf> {
    let pages_dir: PathBuf = frontend_dir.join(PAGES_DIR);
    let Ok(entries) = fs::read_dir(&pages_dir) else {
        return Vec::new();
    };
    let mut packages: Vec<PathBuf> = entries
        .filter_map(|entry: std::io::Result<fs::DirEntry>| entry.ok())
        .map(|entry: fs::DirEntry| entry.path())
        .filter(|path: &PathBuf| path.join(MANIFEST_NAME).is_file())
        .collect();
    packages.sort();
    packages
}

/// Builds one page package and writes its artifacts into the static
/// resource directory.
///
/// The package is skipped when its artifacts are newer than its sources and
/// were built with the same profile, so a rebuild of the server alone does
/// not rebuild the browser assets.
///
/// # Arguments
///
/// - `&Path` - The directory of the server package.
/// - `&Path` - The package directory of the page.
/// - `bool` - Whether the page is built with the release profile.
///
/// # Returns
///
/// - `Result<(), Box<dyn std::error::Error>>` - The page is built, or the
///   build failed.
fn build_page(
    manifest_dir: &Path,
    package: &Path,
    release: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let name: String = match package
        .file_name()
        .and_then(|name: &std::ffi::OsStr| name.to_str())
    {
        Some(name) => name.to_string(),
        None => return Ok(()),
    };
    let output_dir: PathBuf = manifest_dir.join(STATIC_DIR).join(&name);
    let mode: &str = if release {
        PROFILE_RELEASE
    } else {
        PROFILE_DEV
    };
    let shared_dir: PathBuf = manifest_dir.join(FRONTEND_DIR).join(SHARED_DIR);
    if is_fresh(package, &shared_dir, &output_dir, mode) {
        println!("cargo:warning=the page {name} is up to date");
        return Ok(());
    }
    let mut command: Command = Command::new(cli_binary());
    command.arg(CLI_BUILD);
    command.arg(ARG_CRATE_PATH).arg(package);
    command
        .arg(ARG_WWW_DIR)
        .arg(relative_output_dir(manifest_dir, package, &output_dir));
    if release {
        command.arg(ARG_RELEASE);
    } else {
        command.arg(ARG_DEV);
    }
    command.current_dir(package);
    command.env(ENV_TARGET_DIR, frontend_target_dir(manifest_dir));
    command.env_remove(ENV_ENCODED_RUSTFLAGS);
    command.env_remove(ENV_RUSTFLAGS);
    redirect_output(&mut command, &name);
    let status: std::process::ExitStatus = command.status().map_err(|_| MESSAGE_NO_CLI)?;
    if !status.success() {
        return Err(format!("the page {name} could not be built by the {CLI_NAME} command").into());
    }
    check_cli_version(&name)?;
    write_stamp(&name, mode);
    println!(
        "cargo:warning=the page {name} was built into {}",
        output_dir.display()
    );
    Ok(())
}

/// Checks that the command line interface that built a page is the version
/// the project expects.
///
/// The command line interface writes the path of its own sources into its
/// log, and that path carries the version of the crate it was built from.
/// A build with another version would produce artifacts that the pipeline
/// and the images of the server do not produce, so the build stops and says
/// which binary to point at.
///
/// # Arguments
///
/// - `&str` - The name of the page that was built.
///
/// # Returns
///
/// - `Result<(), Box<dyn std::error::Error>>` - The version of the command
///   line interface is the expected one, or the build stops.
fn check_cli_version(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let log_file: PathBuf = log_path(name);
    let Ok(log) = fs::read_to_string(&log_file) else {
        return Ok(());
    };
    let mut versions: Vec<String> = Vec::new();
    let mut rest: &str = log.as_str();
    while let Some(index) = rest.find(CLI_VERSION_PATTERN) {
        rest = &rest[index + CLI_VERSION_PATTERN.len()..];
        let end: usize = rest
            .find(|c: char| !c.is_ascii_digit() && c != '.')
            .unwrap_or(rest.len());
        let version: String = rest[..end].to_string();
        if !version.is_empty() && !versions.contains(&version) {
            versions.push(version);
        }
    }
    for version in versions {
        if version != CLI_VERSION {
            return Err(MESSAGE_CLI_VERSION.into());
        }
    }
    Ok(())
}

/// Returns the target directory the frontend workspace builds into.
///
/// The build script of the server and the build of a page would otherwise
/// share one target directory, and the page build would wait for the lock
/// the server build holds: the server build waits for the page build, and
/// neither of them finishes.
///
/// # Arguments
///
/// - `&Path` - The directory of the server package.
///
/// # Returns
///
/// - `PathBuf` - The target directory of the frontend workspace.
fn frontend_target_dir(manifest_dir: &Path) -> PathBuf {
    manifest_dir.join(FRONTEND_DIR).join(TARGET_DIR)
}

/// Sends the output of the command line interface into the log file.
///
/// The build script writes into a pipe that nobody reads while the script
/// runs, and a build that writes more into it than the pipe holds would
/// block forever. The log file keeps the output of the build readable.
///
/// # Arguments
///
/// - `&mut Command` - The command the output is redirected away from.
/// - `&str` - The name of the page that is built.
fn redirect_output(command: &mut Command, name: &str) {
    let log_file: PathBuf = log_path(name);
    if let Some(parent) = log_file.parent() {
        let created: std::io::Result<()> = create_dir_all(parent);
        let _: std::io::Result<()> = created;
    }
    match File::create(log_file) {
        Ok(log) => match log.try_clone() {
            Ok(errors) => {
                command.stdout(Stdio::from(log));
                command.stderr(Stdio::from(errors));
            }
            Err(_) => {
                command.stdout(Stdio::null());
                command.stderr(Stdio::null());
            }
        },
        Err(_) => {
            command.stdout(Stdio::null());
            command.stderr(Stdio::null());
        }
    }
}

/// Returns the path of the log the frontend build writes to.
///
/// # Arguments
///
/// - `&str` - The name of the page that is built.
///
/// # Returns
///
/// - `PathBuf` - The path of the log.
fn log_path(name: &str) -> PathBuf {
    let out_dir: PathBuf = PathBuf::from(var(ENV_OUT_DIR).unwrap_or_default());
    out_dir.join(STAMP_DIR).join(name).join(LOG_NAME)
}

/// Returns the command line interface binary that builds the pages.
///
/// The binary is taken from the `EUV_CLI` environment variable when it is
/// set, so a machine that keeps the tool outside its path can point the
/// build at it.
///
/// # Returns
///
/// - `String` - The path of the binary.
fn cli_binary() -> String {
    var(ENV_CLI).unwrap_or_else(|_| CLI_NAME.to_string())
}

/// Reports whether the artifacts of a page are newer than its sources.
///
/// The sources of the shared crate are part of every page, so a change in
/// them rebuilds every page as well.
///
/// # Arguments
///
/// - `&Path` - The package directory of the page.
/// - `&Path` - The directory of the shared package.
/// - `&Path` - The directory the artifacts were written to.
/// - `&str` - The profile the artifacts were built with.
///
/// # Returns
///
/// - `bool` - Whether the page can be left alone.
fn is_fresh(package: &Path, shared: &Path, output_dir: &Path, mode: &str) -> bool {
    let index: PathBuf = output_dir.join(INDEX_NAME);
    let Ok(built_at) = index
        .metadata()
        .and_then(|meta: fs::Metadata| meta.modified())
    else {
        return false;
    };
    if stamp_mode(package).as_deref() != Some(mode) {
        return false;
    }
    let page_at: Option<SystemTime> = newest_source_time(package);
    let shared_at: Option<SystemTime> = newest_source_time(shared);
    match (page_at, shared_at) {
        (Some(page_at), Some(shared_at)) => page_at.max(shared_at) <= built_at,
        (Some(page_at), None) => page_at <= built_at,
        (None, Some(shared_at)) => shared_at <= built_at,
        (None, None) => false,
    }
}

/// Reads the profile the artifacts of a page were built with.
///
/// # Arguments
///
/// - `&Path` - The package directory of the page.
///
/// # Returns
///
/// - `Option<String>` - The profile, absent when the page was never built.
fn stamp_mode(package: &Path) -> Option<String> {
    read_to_string(stamp_path(package)).ok()
}

/// Records the profile the artifacts of a page were built with.
///
/// # Arguments
///
/// - `&str` - The name of the page.
/// - `&str` - The profile the page was built with.
///
/// # Returns
///
/// - `()` - The stamp is written, or the write failed and is ignored.
fn write_stamp(name: &str, mode: &str) {
    let stamp: PathBuf = stamp_path(Path::new(name));
    if let Some(parent) = stamp.parent()
        && create_dir_all(parent).is_ok()
    {
        let written: std::io::Result<()> = write(stamp, mode);
        let _: std::io::Result<()> = written;
    }
}

/// Returns the path of the stamp of a page.
///
/// # Arguments
///
/// - `&Path` - The package directory of the page.
///
/// # Returns
///
/// - `PathBuf` - The path of the stamp.
fn stamp_path(package: &Path) -> PathBuf {
    let name: String = package
        .file_name()
        .and_then(|name: &std::ffi::OsStr| name.to_str())
        .unwrap_or_default()
        .to_string();
    let out_dir: PathBuf = PathBuf::from(var(ENV_OUT_DIR).unwrap_or_default());
    out_dir.join(STAMP_DIR).join(format!("{name}.stamp"))
}

/// Returns the moment the newest source of a package was written.
///
/// # Arguments
///
/// - `&Path` - The package directory of the page.
///
/// # Returns
///
/// - `Option<SystemTime>` - The moment, absent when the package holds no source.
fn newest_source_time(package: &Path) -> Option<SystemTime> {
    let mut newest: Option<SystemTime> = None;
    let mut stack: Vec<PathBuf> = vec![package.to_path_buf()];
    while let Some(directory) = stack.pop() {
        let Ok(entries) = fs::read_dir(&directory) else {
            continue;
        };
        for entry in entries.filter_map(|entry: std::io::Result<fs::DirEntry>| entry.ok()) {
            let path: PathBuf = entry.path();
            let Ok(metadata) = fs::metadata(&path) else {
                continue;
            };
            if metadata.is_dir() {
                if path
                    .file_name()
                    .and_then(|name: &std::ffi::OsStr| name.to_str())
                    == Some("target")
                {
                    continue;
                }
                stack.push(path);
                continue;
            }
            let Ok(modified) = metadata.modified() else {
                continue;
            };
            if newest.is_none_or(|current: SystemTime| modified > current) {
                newest = Some(modified);
            }
        }
    }
    newest
}

/// Returns the output directory of a page relative to its package.
///
/// The command line interface resolves the directory against the package it
/// builds, so the path has to be relative to it.
///
/// # Arguments
///
/// - `&Path` - The directory of the server package.
/// - `&Path` - The package directory of the page.
/// - `&Path` - The directory the artifacts are written to.
///
/// # Returns
///
/// - `String` - The relative directory.
fn relative_output_dir(manifest_dir: &Path, package: &Path, output_dir: &Path) -> String {
    let depth: usize = package
        .strip_prefix(manifest_dir)
        .map(|path: &Path| path.components().count())
        .unwrap_or_default();
    let mut relative: String = String::new();
    for _ in 0..depth {
        relative.push_str("../");
    }
    let tail: String = output_dir
        .strip_prefix(manifest_dir)
        .map(|path: &Path| path.to_string_lossy().replace('\\', "/"))
        .unwrap_or_default();
    relative.push_str(&tail);
    relative
}
