pub(crate) mod basic_config;
pub(crate) mod poem_config;
pub(crate) mod error;

use super::*;
use std::fs::{self,File};
use std::io::Read;
use std::path::{PathBuf,Path};


use toml;

pub use self::error::ConfigError;
pub use self::poem_config::PoemConfig;
pub use self::basic_config::BasicConfig;

use crate::environment::{Environment,Environment::*};
use std::collections::HashMap;

const CONFIG_FILENAME: &str = "config/Poem.toml";
pub type Result<T> = ::std::result::Result<T, ConfigError>;
