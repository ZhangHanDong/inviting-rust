use super::*;
use toml::value::{Map, Value};

#[derive(Debug)]
pub struct BasicConfig {
    pub environment: super::Environment,
    pub address: String,
    pub port:  String,
    pub database: Option<Database>,
    pub workers: Option<usize>,
    pub(crate) config_file_path: Option<PathBuf>,
    pub(crate) root_path: Option<PathBuf>,
}

impl BasicConfig {
    pub fn new(env: super::Environment) -> Self {
        Self::default(env)
    }

    pub(crate) fn default(env: super::Environment) -> Self {
        let default_workers = (num_cpus::get() * 2) ;
        let default_config = BasicConfig {
            environment: Development,
            address: "localhost".to_string(),
            port: "8000".to_string(),
            database: None,
            workers: Some(default_workers),
            config_file_path: None,
            root_path: None,
        };
        
        match env {
            Development => {
                BasicConfig {
                    environment: Development,
                    ..default_config
                }
            }
            Staging => {
                BasicConfig {
                    environment: Staging,
                    ..default_config
                }
            }
            Production => {
                BasicConfig {
                    environment: Production,
                    ..default_config
                }
            }
        }
    }

    // Parse Demo： 
    // 
    // {"development": 
    //     Table(
    //         {"address": String("localhost"), 
    //         "database": 
    //             Table({
    //                 "adapter": String("postgresql"), 
    //                 "db_name": String("blog_development"), 
    //                 "pool": Integer(5)
    //             }), 
    //             "port": String("8100"), 
    //             "workers": Integer(4)
    //         }), 
    // "production": 
    //     Table({
    //         "address": String("0.0.0.0"), 
    //         "database": 
    //             Table({"adapter": String("postgresql"), 
    //                     "db_name": String("blog_development"), 
    //                     "pool": Integer(5)
    //             }), 
    //             "port": String("9000")
    //     }), 
    // "staging": 
    //     Table({
    //         "address": String("0.0.0.0"), 
    //         "database": 
    //             Table({
    //                 "adapter": String("postgresql"), 
    //                 "db_name": String("blog_development"), 
    //                 "pool": Integer(5)
    //             }), 
    //             "port": String("9000")
    //     })
    // }
    pub(crate) fn set_config(env: super::Environment, table: &Map<String, Value>) -> Self{
        let default_workers = (num_cpus::get() * 2);
        let default_config = BasicConfig {
            environment: Development,
            address: "localhost".to_string(),
            port: "8000".to_string(),
            database: None,
            workers: Some(default_workers),
            config_file_path: None,
            root_path: None,
        };
        
        match env {
            Development => {
                let value = table.get("development");
                match value {
                    Some(table) => {
                        return BasicConfig {
                            environment: Development,
                            address: table.get("address").and_then(|v|v.as_str()).unwrap().to_string(),
                            port: table.get("port").and_then(|v|v.as_str()).unwrap().to_string(),
                            database: Some(Database::new(table.get("database").unwrap())),
                            workers: table.get("workers").and_then(|v|v.as_integer()).map(|v| v as usize),
                            ..default_config
                        };
                    },
                    None => {panic!("Parse Error")}
                }
                
            }
            Staging => {
                BasicConfig {
                    environment: Staging,
                    // TODO: 留作练习
                    ..default_config
                }
            }
            Production => {
                BasicConfig {
                    // TODO: 留作练习
                    environment: Production,
                    ..default_config
                }
            }
        }
    }

    pub(crate) fn set_root<P: AsRef<Path>>(&mut self, path: P) {
        self.root_path = Some(path.as_ref().into());
    }

    pub(crate) fn default_from<P>(env: super::Environment, path: P) -> super::Result<Self>
        where P: AsRef<Path>
    {
        let mut config = BasicConfig::default(env);

        let config_file_path = path.as_ref().to_path_buf();
        if let Some(parent) = config_file_path.parent() {
            config.set_root(parent);
        } else {
            let msg = "Configuration files must be rooted in a directory.";
            return Err(ConfigError::BadFilePath(config_file_path.clone(), msg));
        }

        config.config_file_path = Some(config_file_path);
        Ok(config)
    }
}

impl PartialEq for BasicConfig {
    fn eq(&self, other: &BasicConfig) -> bool {
        self.address == other.address
            && self.port == other.port
            && self.workers == other.workers
    }
}

#[derive(Debug)]
pub struct Database {
    pub(crate) adapter: String,
    pub(crate) db_name: String,
    pub(crate) pool: usize,
}

    // database": 
    //             Table({
    //                 "adapter": String("postgresql"), 
    //                 "db_name": String("blog_development"), 
    //                 "pool": Integer(5)
    //             }), 
impl Database {
    fn new(table: &Value) -> Self{
        Database {
            adapter: table.get("adapter").and_then(|v|v.as_str()).unwrap().to_string(),
            db_name: table.get("db_name").and_then(|v|v.as_str()).unwrap().to_string(),
            pool: table.get("pool").and_then(|v|v.as_integer()).unwrap() as usize,
        }
    }
}