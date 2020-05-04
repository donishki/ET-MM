use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io:: {
    prelude::*,
    BufReader
};
use std::sync::Arc;

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
    pub database_connection_string: Arc<String>,
    pub discord_token: String,
    pub mm_groups: Arc<Vec<MMGroup>>
}

// Config implmentation
impl Config {
    /// reads the bot configuration parameters from the configuration file
    /// into memory. FIXME: This function is currently hot garbage, but it works.
    ///
    /// # Example
    ///
    /// ```
    /// let db = config::Config::read("config.cfg").unwrap();"
    /// ```
    pub fn construct (path: &str) -> Result<Self, Box<dyn Error>> {
        let config = File::open(path)?;
        let reader = BufReader::new(config);
        let mut db_host: String = String::from("");
        let mut db_user: String = String::from("");
        let mut discord_token: String = String::from("");
        let mut mm_groups: Vec<MMGroup> = Vec::new();

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
                        let tokens: Vec<&str> = line.split(':').collect();
                        if let 2 = tokens.len() {
                            match tokens[0] {
                                "host" => db_host = tokens[1].trim().to_string(),
                                "user" => db_user = tokens[1].trim().to_string(),
                                _ => return Err(format!("unknown key in database section: {}", tokens[0]).into())
                            };
                        };
                    },
                    // parse discord configuration
                    "[discord]" => {
                        let tokens: Vec<&str> = line.split(':').collect();
                        if let 2 = tokens.len() {
                            match tokens[0] {
                                "token" => discord_token = tokens[1].trim().to_string(),
                                _ => return Err(format!("unknown key in discord section: {}", tokens[0]).into())
                            };
                        };
                    },
                    // parse match making groups
                    "[mm-groups]" => mm_groups.push(MMGroup::construct(&line)?),
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
        // return
        Ok (
            Self {
                database_connection_string: Arc::new(format!("host={} user={}", db_host, db_user)), 
                discord_token,
                mm_groups: Arc::new(mm_groups)
            }
        )
    }
}

/// MMGroup structure for match making groups
///
/// # Members
///
///     ```
///     name: name of match making group
///     teams: HashMap containing keys (name of team) and values (number of players per team)
///     players: total number of players required for this match making group
///     ```
pub struct MMGroup {
    pub name: String,
    pub teams: HashMap<String, i32>,
    pub players: i32
}

// MMGroup implementation
impl MMGroup {
    /// creates a MMGroup structure
    ///
    /// # Example
    ///
    /// ```
    /// let line = "6v6: Allies:6, Axis:6";
    /// let mm_group: MMGroup = MMGroup::construct().unwrap();
    /// ```
    fn construct (config_entry: &str) -> Result<Self, Box<dyn Error>> {
        let mut tokens = config_entry.splitn(2, ':');
        let name = match tokens.next() {
            None => return Err(format!("error parsing match making group name in line: {}", config_entry).into()),
            Some(n) => n.trim().to_string()
        };
        let tokens = match tokens.next() {
            None => return Err(format!("error parsing match making group teams in line: {}", config_entry).into()),
            Some(t) => t.to_string()
        };
        let mut teams: HashMap<String, i32> = HashMap::new();
        let tokens: Vec<&str> = tokens.split(',').collect();
        for token in tokens.iter() {
            let tokens: Vec<&str> = token.split(':').collect();
            if let 2 = tokens.len() {
                teams.insert(tokens[0].trim().to_string(), tokens[1].trim().parse::<i32>()?);
            } else {
                return Err(format!("error parsing match making group teams in line: {}", config_entry).into());
            }
        }
        let mut players: i32 = 0;
        for team_players in teams.values() {
            players += team_players;
        }      
        Ok (
            Self {
                name,
                teams,
                players
            }
        )
    }
}
