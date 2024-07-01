use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::SystemTime;

use parking_lot::RwLock;
use serde_json::Value;
use serde_yaml;

use crate::locations::{DEFAULT_CONFIG, DISTRIBUTION_CONFIG, SYSTEM_CONFIG, USER_CONFIG};
use crate::log::{debug, error};

type ConfigDict = HashMap<String, Value>;

#[derive(Clone)]
pub struct LocalConf {
    path: Option<PathBuf>,
    data: Arc<RwLock<ConfigDict>>,
    last_loaded: Arc<RwLock<Option<SystemTime>>>,
}

impl LocalConf {
    pub fn new(path: Option<PathBuf>) -> Self {
        let conf = Self {
            path: path.clone(),
            data: Arc::new(RwLock::new(HashMap::new())),
            last_loaded: Arc::new(RwLock::new(None)),
        };
        if let Some(p) = path {
            conf.load_local(Some(&p));
        }
        conf
    }

    fn get_file_format(&self, path: Option<&Path>) -> &'static str {
        let path = path
            .or_else(|| self.path.as_deref())
            .unwrap_or_else(|| Path::new(""));
        match path.extension().and_then(|s| s.to_str()) {
            Some("yml") | Some("yaml") => "yaml",
            _ => "json",
        }
    }

    pub fn load_local(&self, path: Option<&Path>) {
        let path = path.or_else(|| self.path.as_deref());
        if let Some(path) = path {
            if path.exists() && path.is_file() {
                let config = match self.get_file_format(Some(path)) {
                    "yaml" => {
                        let mut file = File::open(path).expect("Unable to open file");
                        let mut contents = String::new();
                        file.read_to_string(&mut contents)
                            .expect("Unable to read file");
                        serde_yaml::from_str(&contents).expect("Unable to parse YAML")
                    }
                    _ => load_commented_json(path).expect("Unable to load JSON"),
                };
                let mut data = self.data.write();
                for (key, value) in config {
                    data.insert(key, value);
                }
                debug(&format!("Configuration {:?} loaded", path));
                if path == self.path.as_deref().unwrap_or_else(|| Path::new("")) {
                    if let Ok(metadata) = path.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            *self.last_loaded.write() = Some(modified);
                        }
                    }
                }
            } else {
                debug(&format!("Configuration {:?} not defined, skipping", path));
            }
        }
    }

    pub fn reload(&self) {
        if let Some(path) = &self.path {
            if path.is_file() {
                if let Ok(metadata) = path.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        let last_loaded = self.last_loaded.read();
                        if last_loaded.map_or(true, |last| last < modified) {
                            drop(last_loaded); // Release the read lock before calling load_local
                            self.load_local(Some(path));
                        } else {
                            debug(&format!("{:?} not changed since last load", path));
                        }
                    }
                }
            }
        }
    }
    pub fn store(&self, path: Option<&Path>) {
        let path = path.or_else(|| self.path.as_deref());
        if let Some(path) = path {
            let data = self.data.read();
            match self.get_file_format(Some(path)) {
                "yaml" => {
                    let yaml_string =
                        serde_yaml::to_string(&*data).expect("Unable to serialize to YAML");
                    let mut file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(path)
                        .expect("Unable to open file");
                    file.write_all(yaml_string.as_bytes())
                        .expect("Unable to write file");
                }
                _ => {
                    let json_string =
                        serde_json::to_string_pretty(&*data).expect("Unable to serialize to JSON");
                    let mut file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(path)
                        .expect("Unable to open file");
                    file.write_all(json_string.as_bytes())
                        .expect("Unable to write file");
                }
            }
        } else {
            error("In-memory configuration, no save location");
        }
    }

    pub fn merge(&mut self, conf: &ConfigDict) {
        let mut data = self.data.write();
        for (key, value) in conf {
            data.insert(key.clone(), value.clone());
        }
    }
}

pub struct ReadOnlyConfig {
    inner: LocalConf,
    allow_overwrite: bool,
}

impl ReadOnlyConfig {
    pub fn new(path: PathBuf, allow_overwrite: bool) -> Self {
        Self {
            inner: LocalConf::new(Some(path)),
            allow_overwrite,
        }
    }

    pub fn reload(&mut self) {
        let old = self.allow_overwrite;
        self.allow_overwrite = true;
        self.inner.reload();
        self.allow_overwrite = old;
    }

    pub fn set(&mut self, key: &str, value: Value) -> Result<(), &'static str> {
        if !self.allow_overwrite {
            Err("This configuration is read-only and cannot be modified at runtime")
        } else {
            self.inner.data.write().insert(key.to_string(), value);
            Ok(())
        }
    }

    pub fn merge(&mut self, conf: &ConfigDict) -> Result<(), &'static str> {
        if !self.allow_overwrite {
            Err("This configuration is read-only and cannot be modified at runtime")
        } else {
            self.inner.merge(conf);
            Ok(())
        }
    }

    pub fn store(&self, path: Option<&Path>) -> Result<(), &'static str> {
        if !self.allow_overwrite {
            Err("This configuration is read-only and cannot be modified at runtime")
        } else {
            self.inner.store(path);
            Ok(())
        }
    }
}

pub struct MycroftDefaultConfig(ReadOnlyConfig);

impl MycroftDefaultConfig {
    pub fn new() -> Self {
        Self(ReadOnlyConfig::new(DEFAULT_CONFIG.to_path_buf(), false))
    }

    pub fn set_root_config_path(&mut self, root_config: PathBuf) {
        self.0.inner.path = Some(root_config);
        self.0.reload();
    }
}

pub struct OvosDistributionConfig(ReadOnlyConfig);

impl OvosDistributionConfig {
    pub fn new(allow_overwrite: bool) -> Self {
        Self(ReadOnlyConfig::new(
            DISTRIBUTION_CONFIG.to_path_buf(),
            allow_overwrite,
        ))
    }
}

pub struct MycroftSystemConfig(ReadOnlyConfig);

impl MycroftSystemConfig {
    pub fn new(allow_overwrite: bool) -> Self {
        Self(ReadOnlyConfig::new(
            SYSTEM_CONFIG.to_path_buf(),
            allow_overwrite,
        ))
    }
}

pub struct MycroftUserConfig(LocalConf);

impl MycroftUserConfig {
    pub fn new() -> Self {
        Self(LocalConf::new(Some(USER_CONFIG.to_path_buf())))
    }
}

pub type MycroftXDGConfig = MycroftUserConfig;

// Helper function to load JSON with comments
fn load_commented_json(path: &Path) -> Result<ConfigDict, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Remove comments (both single-line and multi-line)
    let re = regex::Regex::new(r"(/\*([^*]|[\r\n]|(\*+([^*/]|[\r\n])))*\*+/)|(//.*)")?;
    let json_str = re.replace_all(&contents, "");

    let config: ConfigDict = serde_json::from_str(&json_str)?;
    Ok(config)
}
