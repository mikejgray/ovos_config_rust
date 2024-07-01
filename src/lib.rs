//! OVOS Configuration Reader
//!
//! This crate provides functionality to read and parse OVOS configuration files
//! from XDG-compliant directories.
//!
//! It includes an implementation of the XDG Base Directory Specification.
//!
//! # Examples
//!
//! Using XDG directory functions:
//!
//! ```
//! use ovos_config::xdg;
//!
//! let config_home = xdg::xdg_config_home();
//! println!("XDG config home: {:?}", config_home);
//! ```

pub mod config;
pub mod locations;
pub mod log;
pub mod xdg;
