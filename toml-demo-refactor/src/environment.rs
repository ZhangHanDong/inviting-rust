
use super::*;
use std::str::FromStr;

use crate::conf::ConfigError;
use self::Environment::*;

pub const CONFIG_ENV: &str = "POEM_ENV";

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Environment {
    /// The development environment. for Debug mode.
    Development,
    /// The staging environment. for Debug mode.
    Staging,
    /// The production environment. for Release mode.
    Production,
}

impl Environment {
    /// List of all of the possible environments.
    pub(crate) const ALL: [Environment; 3] = [Development, Staging, Production];

    /// String of all valid environments.
    pub(crate) const VALID: &'static str = "development, staging, production";

    pub fn active() -> Result<Environment, ConfigError> {
        match env::var(CONFIG_ENV) {
            Ok(s) => s.parse().map_err(|_| ConfigError::BadEnv(s)),
            // for Debug mode
            #[cfg(debug_assertions)]
            _ => Ok(Development),
            // for Release mode
            #[cfg(not(debug_assertions))]
            _ => Ok(Production),
        }
    }

    #[inline]
    pub fn is_dev(self) -> bool {
        self == Development
    }

    #[inline]
    pub fn is_stage(self) -> bool {
        self == Staging
    }

    #[inline]
    pub fn is_prod(self) -> bool {
        self == Production
    }
}

impl FromStr for Environment {
    type Err = ();

    /// Parsing a production environment:
    ///
    /// ```rust
    /// let env = "p".parse::<Environment>();
    /// assert_eq!(env.unwrap(), Environment::Production);
    ///
    /// let env = "prod".parse::<Environment>();
    /// assert_eq!(env.unwrap(), Environment::Production);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let env = match s {
            "d" | "dev" | "devel" | "development" => Development,
            "s" | "stage" | "staging" => Staging,
            "p" | "prod" | "production" => Production,
            _ => return Err(()),
        };

        Ok(env)
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Development => write!(f, "development"),
            Staging => write!(f, "staging"),
            Production => write!(f, "production"),
        }
    }
}