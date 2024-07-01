//! XDG Base Directory Specification implementation.
//!
//! This module provides functions to retrieve paths and directories
//! as specified by the [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html).
//!
//! It includes functions to get the XDG cache, config, and data directories,
//! as well as the runtime directory.

use std::env;
use std::path::{Path, PathBuf};

/// Returns the path to the XDG cache home directory.
///
/// This function follows the XDG Base Directory Specification. It returns a `PathBuf` containing:
/// - The value of the `XDG_CACHE_HOME` environment variable if it is set and not empty.
/// - `~/.cache` if `XDG_CACHE_HOME` is not set, empty, or contains a relative path.
///
/// # Examples
///
/// ```
/// use ovos_config::xdg;
///
/// let cache_home = xdg::xdg_cache_home();
/// println!("XDG cache home: {:?}", cache_home);
/// ```
pub fn xdg_cache_home() -> PathBuf {
    path_from_env("XDG_CACHE_HOME", || home_dir().join(".cache"))
}

/// Returns a list of paths to the XDG config directories.
///
/// This function follows the XDG Base Directory Specification. It returns a `Vec<PathBuf>` containing:
/// - The values from the `XDG_CONFIG_DIRS` environment variable, split on colons.
/// - `["/etc/xdg"]` if `XDG_CONFIG_DIRS` is not set or empty.
///
/// Relative paths are ignored, as per the specification.
///
/// # Examples
///
/// ```
/// use ovos_config::xdg;
///
/// let config_dirs = xdg::xdg_config_dirs();
/// for dir in config_dirs {
///     println!("Config dir: {:?}", dir);
/// }
/// ```
pub fn xdg_config_dirs() -> Vec<PathBuf> {
    paths_from_env("XDG_CONFIG_DIRS", || vec![PathBuf::from("/etc/xdg")])
}

/// Returns the path to the XDG config home directory.
///
/// This function follows the XDG Base Directory Specification. It returns a `PathBuf` containing:
/// - The value of the `XDG_CONFIG_HOME` environment variable if it is set and not empty.
/// - `~/.config` if `XDG_CONFIG_HOME` is not set, empty, or contains a relative path.
///
/// # Examples
///
/// ```
/// use ovos_config::xdg;
///
/// let config_home = xdg::xdg_config_home();
/// println!("XDG config home: {:?}", config_home);
/// ```
pub fn xdg_config_home() -> PathBuf {
    path_from_env("XDG_CONFIG_HOME", || home_dir().join(".config"))
}

/// Returns a list of paths to the XDG data directories.
///
/// This function follows the XDG Base Directory Specification. It returns a `Vec<PathBuf>` containing:
/// - The values from the `XDG_DATA_DIRS` environment variable, split on colons.
/// - `["/usr/local/share", "/usr/share"]` if `XDG_DATA_DIRS` is not set or empty.
///
/// Relative paths are ignored, as per the specification.
///
/// # Examples
///
/// ```
/// use ovos_config::xdg;
///
/// let data_dirs = xdg::xdg_data_dirs();
/// for dir in data_dirs {
///     println!("Data dir: {:?}", dir);
/// }
/// ```
pub fn xdg_data_dirs() -> Vec<PathBuf> {
    paths_from_env("XDG_DATA_DIRS", || {
        vec![
            PathBuf::from("/usr/local/share"),
            PathBuf::from("/usr/share"),
        ]
    })
}

/// Returns the path to the XDG data home directory.
///
/// This function follows the XDG Base Directory Specification. It returns a `PathBuf` containing:
/// - The value of the `XDG_DATA_HOME` environment variable if it is set and not empty.
/// - `~/.local/share` if `XDG_DATA_HOME` is not set, empty, or contains a relative path.
///
/// # Examples
///
/// ```
/// use ovos_config::xdg;
///
/// let data_home = xdg::xdg_data_home();
/// println!("XDG data home: {:?}", data_home);
/// ```
pub fn xdg_data_home() -> PathBuf {
    path_from_env("XDG_DATA_HOME", || home_dir().join(".local").join("share"))
}

/// Returns the path to the XDG runtime directory.
///
/// This function follows the XDG Base Directory Specification. It returns an `Option<PathBuf>` containing:
/// - The value of the `XDG_RUNTIME_DIR` environment variable if it is set and not empty.
/// - `None` if `XDG_RUNTIME_DIR` is not set, empty, or contains a relative path.
///
/// # Examples
///
/// ```
/// use ovos_config::xdg;
///
/// if let Some(runtime_dir) = xdg::xdg_runtime_dir() {
///     println!("XDG runtime dir: {:?}", runtime_dir);
/// } else {
///     println!("XDG runtime dir is not set");
/// }
/// ```
pub fn xdg_runtime_dir() -> Option<PathBuf> {
    env::var_os("XDG_RUNTIME_DIR")
        .and_then(|os_str| os_str.into_string().ok())
        .and_then(|s| {
            if Path::new(&s).is_absolute() {
                Some(s.into())
            } else {
                None
            }
        })
}

/// Returns the path to the XDG state home directory.
///
/// This function follows the XDG Base Directory Specification. It returns a `PathBuf` containing:
/// - The value of the `XDG_STATE_HOME` environment variable if it is set and not empty.
/// - `~/.local/state` if `XDG_STATE_HOME` is not set, empty, or contains a relative path.
///
/// # Examples
///
/// ```
/// use ovos_config::xdg;
///
/// let state_home = xdg::xdg_state_home();
/// println!("XDG state home: {:?}", state_home);
/// ```
pub fn xdg_state_home() -> PathBuf {
    path_from_env("XDG_STATE_HOME", || home_dir().join(".local").join("state"))
}

/// Helper function to get a path from an environment variable or use a default.
fn path_from_env<F>(var: &str, default: F) -> PathBuf
where
    F: FnOnce() -> PathBuf,
{
    env::var_os(var)
        .and_then(|os_str| os_str.into_string().ok())
        .filter(|s| !s.is_empty() && Path::new(s).is_absolute())
        .map(PathBuf::from)
        .unwrap_or_else(default)
}

/// Helper function to get a list of paths from an environment variable or use a default.
fn paths_from_env<F>(var: &str, default: F) -> Vec<PathBuf>
where
    F: FnOnce() -> Vec<PathBuf>,
{
    env::var_os(var)
        .and_then(|os_str| os_str.into_string().ok())
        .map(|s| {
            s.split(':')
                .filter(|path| !path.is_empty() && Path::new(path).is_absolute())
                .map(PathBuf::from)
                .collect()
        })
        .filter(|paths: &Vec<PathBuf>| !paths.is_empty())
        .unwrap_or_else(default)
}

/// Helper function to get the user's home directory.
fn home_dir() -> PathBuf {
    env::var_os("HOME")
        .and_then(|h| if h.is_empty() { None } else { Some(h) })
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/"))
}
