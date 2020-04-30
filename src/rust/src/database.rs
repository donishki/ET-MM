use crate::logger::Log;
use postgres:: {
    Client,
    NoTls,
    types::Type
};
use std:: {
    error::Error,
    sync::Arc
};


/// Database structure
///
/// # Members
///
///     ```
///     connection_string: reference to database connection string
///     logger: reference to application logger
///     ```
pub struct Database {
    connection_string: String,
    log: Arc<Log>
}

// Database implmentation
impl Database {
    /// connects to postgresql and constructs the database object.
    /// FIXME: uses a copy since lifetime constraints are too tricky
    ///        when later used with the serenity crate. Not a big deal
    ///        to be honest.
    ///
    /// # Example
    ///
    /// ```
    /// let db = database::Database::construct("host=localhost user=user").unwrap();"
    /// ```
    pub fn construct (connection_string: &String, log: &Arc<Log>) -> Result<Self, Box<dyn Error>> {
        Client::connect(&connection_string, NoTls)?;
        Ok (
            Self {
                connection_string: connection_string.to_string(),
                log: Arc::clone(&log)
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
        let mut client = Client::connect(&self.connection_string, NoTls)?;
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
                _ => return Err(format!("unknown database result for add_match_making_groups function: {}", result).into())
            };
        }
        Ok (())
    }
    /// adds user to specified match making group in the database for a given 
    /// discord uuid and group name. this is done by calling the add_match_making_user()
    /// stored psql function. the database returns 0 on success, 1 on failure due to
    /// failure to add the specified user to the database, or 2 on failure due to the
    /// user already belonging to the specified group.
    ///
    /// #FIXME: We have to cast the u64 to a string here since the postgres lib can't
    ///         convert a u64 to NUMERIC 
    ///
    /// # Example
    ///
    /// ```
    /// database::Database::add_mm_user("uuid", "1v1").unwrap();"
    /// ```
    pub fn add_mm_user (&self, discord_uuid: &u64, group: &str) -> Result <i32, Box<dyn Error>> {
        let mut client = Client::connect(&self.connection_string, NoTls)?;
        let statement = client.prepare_typed (
            "SELECT add_match_making_user ( $1, $2 );",
            &[Type::TEXT, Type::TEXT]
        )?;
        let rows = client.query(&statement, &[&discord_uuid.to_string(), &group])?;
        Ok (rows[0].get(0))
        // match result {
        //     0 => info!(self.log.logger, "\tadded user to group"; "user" => discord_uuid, "group" => group),
        //     1 => warn!(self.log.logger, "\tfailed to add user to database"; "user" => discord_uuid),
        //     2 => warn!(self.log.logger, "\tuser already belongs to group"; "user" => discord_uuid, "group" => group),
        //     _ => return Err(format!("unknown database result for add_match_making_user function: {}", result).into())
        // };
        // return 
        // Ok (())
    }
}
