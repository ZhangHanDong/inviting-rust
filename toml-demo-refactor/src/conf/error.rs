
use super::*;
use std::error::Error;
use self::ConfigError::*;

#[derive(Debug)]
pub enum ConfigError {
    /// The configuration file was not found.
    NotFound,
    /// There was an I/O error while reading the configuration file.
    IoError,
    /// The path at which the configuration file was found was invalid.
    BadFilePath(PathBuf, &'static str),
    /// An environment specified in `POEM_ENV` is invalid.
    BadEnv(String),
    /// An environment specified as a table `[environment]` is invalid.
    BadEntry(String, PathBuf),
    /// A config key was specified with a value of the wrong type.
    BadType(String, &'static str, &'static str, Option<PathBuf>),
    /// There was a TOML parsing error.
    ParseError(String, PathBuf, String, Option<(usize, usize)>),
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            NotFound => "config file was not found",
            IoError => "there was an I/O error while reading the config file",
            BadFilePath(..) => "the config file path is invalid",
            BadEnv(..) => "the environment specified in `ROCKET_ENV` is invalid",
            BadEntry(..) => "an environment specified as `[environment]` is invalid",
            BadType(..) => "a key was specified with a value of the wrong type",
            ParseError(..) => "the config file contains invalid TOML",
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NotFound => write!(f, "config file was not found"),
            IoError => write!(f, "I/O error while reading the config file"),
            BadFilePath(ref p, _) => write!(f, "{:?} is not a valid config path", p),
            BadEnv(ref e) => write!(f, "{:?} is not a valid `ROCKET_ENV` value", e),
            BadEntry(ref e, _) => {
                write!(f, "{:?} is not a valid `[environment]` entry", e)
            }
            BadType(ref n, e, a, _) => {
                write!(f, "type mismatch for '{}'. expected {}, found {}", n, e, a)
            }
            ParseError(..) => write!(f, "the config file contains invalid TOML"),
        }
    }
}