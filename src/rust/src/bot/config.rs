use std::error::Error;
use std::fs::File;
use std::io:: {
    prelude::*,
    BufReader
};

/// Config structure for bot configuration
///
/// # Members
///
///     ```
///     db_connection_string: string for connecting to postgres database
///     discord_token: token for discord bot api
///     mm_groups: match making groups as defined by configuration file 
///     ```
pub struct Config {
    pub db_connection_string: String,
    pub mm_groups: Vec<String>
}

// Config implmentation
impl Config {
    /// reads the bot configuration parameters from the configuration file
    /// into memory. FIXME: This function is currently hot garbage, but it works.
    ///
    /// # Example
    ///
    /// ```
    /// let db = config::Config::readconfig("config.cfg").unwrap();"
    /// ```
    pub fn read_config (path: &str) -> Result<Self, Box<dyn Error>> {
        let config = File::open(path)?;
        let reader = BufReader::new(config);
        let mut db_host: String = String::from("");
        let mut db_user: String = String::from("");
        let mut discord_token: String = String::from("");
        let mut mm_groups: Vec<String> = Vec::new();

        // parse the configuration file
        // FIXME: Pretty tired; no way any of this is idiomatic, but it will work
        //        unless sombody is explicitly trying to break it.
        let mut section_name: String = String::from("");
        for line in reader.lines() {
            let line = line?;
            if line.starts_with('[') && line.ends_with(']') {
               section_name = String::from(&line.to_owned());
            }
            if line != section_name && section_name != "" {
                // parse database configuration
                match section_name.as_ref() {
                    // parse database settings
                    "[database]" => {
                        let tokens: Vec<&str> = line
                            .split(':')
                            .collect();
                        if let 2 = tokens.len() {
                            match tokens[0] {
                                "host" => db_host = tokens[1]
                                    .trim()
                                    .to_string(),
                                "user" => db_user = tokens[1]
                                    .trim()
                                    .to_string(),
                                _ => return Err(format!("unknown key in database section: {}", tokens[0]).into())
                            };
                        };
                    },
                    // parse discord configuration
                    "[discord]" => {
                        let tokens: Vec<&str> = line
                            .split(':')
                            .collect();
                        if let 2 = tokens.len() {
                            match tokens[0] {
                                "token" => discord_token = tokens[1]
                                    .trim()
                                    .to_string(),
                                _ => return Err(format!("unknown key in discord section: {}", tokens[0]).into())
                            };
                        };
                    },
                    // parse match making groups
                    "[mm-groups]" => mm_groups.push(line),
                    _ => return Err(format!("unknown section in file: {}", section_name).into())
                };
            }
        }
        // verify configuration
        if db_host.is_empty() {
            return Err("database information: db_host not in configuration file".into());
        } else if db_user.is_empty() {
            return Err("database information: db_user not in configuration file".into());
        } else if discord_token.is_empty() {
            return Err("discord information: token not in configuration file".into());
        } else if mm_groups.is_empty() {
            return Err("match making group information: no match making groups in configuration file".into());
        }
        // build db_connection_string
        let db_connection_string: String = format!("host={} user={}", db_host, db_user);
        // return
        Ok (
            Self {
                db_connection_string,
                mm_groups
            }
        )
    }
}
