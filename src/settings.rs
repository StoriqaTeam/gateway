use std::env;
use config::{Config, ConfigError, Environment, File};


pub fn get() -> Result<Config, ConfigError> {
    let mut s = Config::new();

    s.merge(File::with_name("config/base"))?;

    // Note that this file is _optional_
    let env = env::var("RUN_MODE").unwrap_or("development".into());
    s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

    // Add in settings from the environment (with a prefix of STQ_GATEWAY)
    s.merge(Environment::with_prefix("STQ_GATEWAY"))?;

    Ok(s)
}
