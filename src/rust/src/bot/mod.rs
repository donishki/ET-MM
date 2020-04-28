use std::error::Error;

mod config;

/// Bot structure for bot instance
///
/// # Members
///
///     ```
///     config: loaded configuration settings
///     ```
pub struct Bot {
    pub config: config::Config
}

impl Bot {
    /// constructs a new bot instance
    ///
    /// # Example
    ///
    /// ```
    /// let bot = bot::construct("config.cfg").unwrap();"
    /// ```
    pub fn construct (config_path: &str) -> Result<Self, Box<dyn Error>> {
        let config = config::Config::read_config(config_path)?;
        Ok (
            Self {
                config: config
            }
        ) 
    }
}