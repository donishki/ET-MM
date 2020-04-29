use postgres:: {
    Client,
    NoTls,
    types::Type
};
use crate::logger::Log;
use std::error::Error;

/// Database structure
///
/// # Members
///
///     ```
///     connection_string: reference to database connection string
///     logger: reference to application logger
///     ```
pub struct Database<'a> {
    connection_string: &'a str,
    log: &'a Log
}

// Database implmentation
impl <'a>Database<'a> {
    /// connects to postgresql and constructs the database object.
    ///
    /// # Example
    ///
    /// ```
    /// let db = database::Database::construct("host=localhost user=user").unwrap();"
    /// ```
    pub fn construct (connection_string: &'a str, log: &'a Log) -> Result<Self, Box<dyn Error>> {
        Client::connect(connection_string, NoTls)?;
        Ok (
            Self {
                connection_string,
                log
            }
        )
    }
    /// adds specified match making groups to the database for a given 
    /// vector of groups. this is done by calling the add_matchmaking_groups()
    /// stored psql function. the database returns 0 on success or 1 on failure
    /// due to the group already existing. 
    ///
    /// # Example
    ///
    /// ```
    /// let groups: Vec<String> = Vec::new();
    /// groups.push("1v1");
    /// groups.push("3v3");
    /// groups.push("6v6");
    /// database::Database::add_mm_groups(groups).unwrap();"
    /// ```
    pub fn add_mm_groups (&self, groups: &[String]) -> Result <(), Box<dyn Error>> {
        let mut client = Client::connect(self.connection_string, NoTls)?;
        for group in groups.iter() {
            let statement = client.prepare_typed (
                "SELECT add_match_making_group ( $1 );",
                &[Type::TEXT]
            )?;
            let rows = client.query(&statement, &[&group])?;
            let result: i32 = rows[0].get(0);
            match result {
                0 => info!(self.log.logger, "\tadded group"; "group" => group),
                1 => warn!(self.log.logger, "\tgroup already exists in database"; "group" => group),
                _ => return Err(format!("unknown database result for add_matchmaking_groups function: {}", result).into())
            };
        }
        Ok (())
    }
}
