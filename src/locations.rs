use crate::xdg;
use std::env;
use std::path::{Path, PathBuf};

/// Returns a list of possible XDG config directories for the given folder.
///
/// # Arguments
///
/// * `folder` - An optional folder name. If None, uses the default XDG base.
///
/// # Examples
///
/// ```
/// use mycroft_config::config_locations;
///
/// let config_dirs = config_locations::get_xdg_config_dirs(None);
/// println!("XDG config dirs: {:?}", config_dirs);
/// ```
pub fn get_xdg_config_dirs(folder: Option<&str>) -> Vec<PathBuf> {
    let folder = folder.unwrap_or("mycroft");
    let mut xdg_dirs = xdg::xdg_config_dirs();
    xdg_dirs.push(xdg::xdg_config_home());
    xdg_dirs.into_iter().map(|path| path.join(folder)).collect()
}

/// Returns a list of possible XDG data directories for the given folder.
///
/// # Arguments
///
/// * `folder` - An optional folder name. If None, uses the default XDG base.
///
/// # Examples
///
/// ```
/// use mycroft_config::config_locations;
///
/// let data_dirs = config_locations::get_xdg_data_dirs(None);
/// println!("XDG data dirs: {:?}", data_dirs);
/// ```
pub fn get_xdg_data_dirs(folder: Option<&str>) -> Vec<PathBuf> {
    let folder = folder.unwrap_or("mycroft");
    xdg::xdg_data_dirs()
        .into_iter()
        .map(|path| path.join(folder))
        .collect()
}

/// Returns the XDG config save path for the given folder.
///
/// # Arguments
///
/// * `folder` - An optional folder name. If None, uses the default XDG base.
///
/// # Examples
///
/// ```
/// use mycroft_config::config_locations;
///
/// let config_save_path = config_locations::get_xdg_config_save_path(None);
/// println!("XDG config save path: {:?}", config_save_path);
/// ```
pub fn get_xdg_config_save_path(folder: Option<&str>) -> PathBuf {
    let folder = folder.unwrap_or("mycroft");
    xdg::xdg_config_home().join(folder)
}

/// Returns the XDG data save path for the given folder.
///
/// # Arguments
///
/// * `folder` - An optional folder name. If None, uses the default XDG base.
///
/// # Examples
///
/// ```
/// use mycroft_config::config_locations;
///
/// let data_save_path = config_locations::get_xdg_data_save_path(None);
/// println!("XDG data save path: {:?}", data_save_path);
/// ```
pub fn get_xdg_data_save_path(folder: Option<&str>) -> PathBuf {
    let folder = folder.unwrap_or("mycroft");
    xdg::xdg_data_home().join(folder)
}

/// Returns the XDG cache save path for the given folder.
///
/// # Arguments
///
/// * `folder` - An optional folder name. If None, uses the default XDG base.
///
/// # Examples
///
/// ```
/// use mycroft_config::config_locations;
///
/// let cache_save_path = config_locations::get_xdg_cache_save_path(None);
/// println!("XDG cache save path: {:?}", cache_save_path);
/// ```
pub fn get_xdg_cache_save_path(folder: Option<&str>) -> PathBuf {
    let folder = folder.unwrap_or("mycroft");
    xdg::xdg_cache_home().join(folder)
}

/// Returns the user config file path.
///
/// # Examples
///
/// ```
/// use mycroft_config::config_locations;
///
/// let user_config = config_locations::find_user_config();
/// println!("User config path: {:?}", user_config);
/// ```
pub fn find_user_config() -> PathBuf {
    let path = get_xdg_config_save_path(None).join("mycroft.conf");
    if path.is_file() {
        path
    } else {
        let old_path = Path::new(&env::var("HOME").unwrap_or_else(|_| String::from("/")))
            .join(".mycroft/mycroft.conf");
        if old_path.is_file() {
            old_path
        } else {
            path
        }
    }
}

/// Returns a list of all possible config file paths.
///
/// # Examples
///
/// ```
/// use mycroft_config::config_locations;
///
/// let config_locations = config_locations::get_config_locations();
/// println!("Config locations: {:?}", config_locations);
/// ```
pub fn get_config_locations() -> Vec<PathBuf> {
    let mut locs = Vec::new();

    // Default config
    locs.push(PathBuf::from("/etc/mycroft/mycroft.conf"));

    // Distribution config
    locs.push(PathBuf::from("/usr/share/mycroft/mycroft.conf"));

    // System config
    locs.push(PathBuf::from("/etc/mycroft/mycroft.conf"));

    // Web cache
    locs.push(get_webcache_location());

    // Old user config
    locs.push(
        Path::new(&env::var("HOME").unwrap_or_else(|_| String::from("/")))
            .join(".mycroft/mycroft.conf"),
    );

    // User config
    locs.push(get_xdg_config_save_path(None).join("mycroft.conf"));

    locs
}

/// Returns the webcache location.
///
/// # Examples
///
/// ```
/// use mycroft_config::config_locations;
///
/// let webcache_location = config_locations::get_webcache_location();
/// println!("Webcache location: {:?}", webcache_location);
/// ```
pub fn get_webcache_location() -> PathBuf {
    get_xdg_config_save_path(None).join("web_cache.json")
}

/// Returns a list of XDG config locations.
///
/// # Examples
///
/// ```
/// use mycroft_config::config_locations;
///
/// let xdg_config_locations = config_locations::get_xdg_config_locations();
/// println!("XDG config locations: {:?}", xdg_config_locations);
/// ```
pub fn get_xdg_config_locations() -> Vec<PathBuf> {
    get_xdg_config_dirs(None)
        .into_iter()
        .map(|p| p.join("mycroft.conf"))
        .rev()
        .collect()
}

/// Returns the path to the default config file.
///
/// # Examples
///
/// ```
/// use mycroft_config::config_locations;
///
/// let default_config = config_locations::find_default_config();
/// println!("Default config path: {:?}", default_config);
/// ```
pub fn find_default_config() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/mycroft.conf")
}

lazy_static::lazy_static! {
    pub static ref DEFAULT_CONFIG: PathBuf = PathBuf::from("/etc/mycroft/mycroft.conf");
    pub static ref DISTRIBUTION_CONFIG: PathBuf = PathBuf::from(env::var("OVOS_DISTRIBUTION_CONFIG")
        .unwrap_or_else(|_| String::from("/usr/share/mycroft/mycroft.conf")));
    pub static ref SYSTEM_CONFIG: PathBuf = PathBuf::from(env::var("MYCROFT_SYSTEM_CONFIG")
        .unwrap_or_else(|_| String::from("/etc/mycroft/mycroft.conf")));
    pub static ref OLD_USER_CONFIG: PathBuf = Path::new(&env::var("HOME").unwrap_or_else(|_| String::from("/")))
        .join(".mycroft/mycroft.conf");
    pub static ref USER_CONFIG: PathBuf = get_xdg_config_save_path(None).join("mycroft.conf");
    pub static ref REMOTE_CONFIG: &'static str = "mycroft.ai";
    pub static ref WEB_CONFIG_CACHE: PathBuf = PathBuf::from(env::var("MYCROFT_WEB_CACHE")
        .unwrap_or_else(|_| get_webcache_location().to_string_lossy().into_owned()));
}

/// Ensures that the directory for the specified path exists.
///
/// # Arguments
///
/// * `path` - The path to the config file
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use mycroft_config::config_locations;
///
/// config_locations::ensure_folder_exists(&PathBuf::from("/tmp/mycroft/test.conf"));
/// ```
pub fn ensure_folder_exists(path: &Path) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
}
