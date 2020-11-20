
use super::*;

#[doc(hidden)]
#[derive(Debug, PartialEq)]
pub struct PoemConfig {
    pub active_env: Environment,
    config: HashMap<Environment, BasicConfig>,
}

impl PoemConfig {

    pub fn read_config() ->  super::Result<PoemConfig> {
        let file = PoemConfig::find()?;

        // Try to open the config file for reading.
        let mut handle = File::open(&file).map_err(|_| ConfigError::IoError)?;

        let mut contents = String::new();
        handle.read_to_string(&mut contents).map_err(|_| ConfigError::IoError)?;

        PoemConfig::parse(contents, &file)
    }

    fn find() -> super::Result<PathBuf> {
        let cwd = env::current_dir().map_err(|_| ConfigError::NotFound)?;
        let mut current = cwd.as_path();

        loop {
            let manifest = current.join(super::CONFIG_FILENAME);
            if fs::metadata(&manifest).is_ok() {
                return Ok(manifest)
            }

            match current.parent() {
                Some(p) => current = p,
                None => break,
            }
        }

        Err(ConfigError::NotFound)
    }

    fn get_mut(&mut self, env: Environment) -> &mut BasicConfig {
        match self.config.get_mut(&env) {
            Some(config) => config,
            None => panic!("set(): {} config is missing.", env),
        }
    }

    pub fn active_default_from(filename: Option<&Path>) -> super::Result<PoemConfig> {
        let mut defaults = HashMap::new();
        if let Some(path) = filename {
            defaults.insert(Development, BasicConfig::default_from(Development, &path)?);
            defaults.insert(Staging, BasicConfig::default_from(Staging, &path)?);
            defaults.insert(Production, BasicConfig::default_from(Production, &path)?);
        } else {
            defaults.insert(Development, BasicConfig::default(Development));
            defaults.insert(Staging, BasicConfig::default(Staging));
            defaults.insert(Production, BasicConfig::default(Production));
        }

        let mut config = PoemConfig {
            active_env: Environment::active()?,
            config: defaults,
        };

        Ok(config)
    }

    pub fn active() -> super::Result<BasicConfig> {
        Ok(BasicConfig::new(Environment::active()?))
    }

    fn parse<P: AsRef<Path>>(src: String, filename: P) -> super::Result<PoemConfig> {
        let path = filename.as_ref().to_path_buf();
        let table = match src.parse::<toml::Value>() {
            Ok(toml::Value::Table(table)) => table,
            Ok(value) => {
                let err = format!("expected a table, found {}", value.type_str());
                return Err(ConfigError::ParseError(src, path, err, Some((1, 1))));
            }
            Err(e) => return Err(ConfigError::ParseError(src, path, e.to_string(), e.line_col()))
        };

        // Create a config with the defaults; set the env to the active one.
        let mut config = PoemConfig::active_default_from(Some(filename.as_ref()))?;


        // Parse the values from the TOML file.
        for (entry, value) in table {
            // Each environment must be a table.
            let kv_pairs = match value.as_table() {
                Some(table) => table,
                None => return Err(ConfigError::BadType(
                    entry, "a table", value.type_str(), Some(path.clone())
                ))
            };

        }

        Ok(config)
    }

}