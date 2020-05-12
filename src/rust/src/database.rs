use crate::config::Config;
use crate::logger::Log;
use tokio_postgres:: {
    NoTls,
    types::Type
};
use std:: {
    error::Error,
    sync::Arc
};
use tokio::sync::RwLock;


/// Database structure
///
/// # Members
///
///     ```
///     connection_string: reference to database connection string
///     logger: reference to application logger
///     ```
pub struct Database {
    connection_string: Arc<String>,
    log: Arc<RwLock<Log>>
}

// Database implmentation
impl Database {
    /// connects to postgresql (just to test) and constructs the database object.
    /// FIXME: uses atomic clone since lifetime constraints are too tricky
    ///        when later used with the serenity crate. Not a big deal
    ///        to be honest.
    ///
    /// # Example
    ///
    /// ```
    /// let config = config::Config::construct("/opt/et-mm-bot/config.cfg").unwrap();
    /// let db = database::Database::construct(&config).unwrap();"
    /// ```
    pub async fn construct (config: &Config, log: &Arc<RwLock<Log>>) -> Result<Self, Box<dyn Error>> {
        let _ = tokio_postgres::connect(&config.database_connection_string, NoTls).await?;
        Ok (
            Self {
                connection_string: Arc::clone(&config.database_connection_string),
                log: Arc::clone(&log)
            }
        )
    }
    /// adds specified match making groups to the database for a given 
    /// vector of groups. this is done by calling the add_matchmaking_groups()
    /// stored function.
    ///
    /// the stored function returns the following:
    ///     0: success
    ///     1: match making group already exists
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
    pub async fn add_mm_groups (&self, config: &Config) -> Result <(), Box<dyn Error>> {
        let (client, connection) = tokio_postgres::connect(&self.connection_string, NoTls).await?;
        let log = self.log.read();
        let groups = config.mm_groups.read();
        tokio::spawn(async move {
            connection.await
        });
        let groups = groups.await;
        let log = log.await;
        for group in groups.iter() {
            let group = &group.name;
            let statement = client.prepare_typed (
                "SELECT add_match_making_group ( $1 );",
                &[Type::TEXT]
            ).await?;
            let rows = client.query(&statement, &[&group]).await?;
            let result: i32 = rows[0].get(0);
            match result {
                0 => info!(log.logger, "\tadded group"; "group" => group),
                1 => warn!(log.logger, "\tgroup already exists in database"; "group" => group),
                _ => return Err(format!("unknown database result for add_match_making_groups function: {}", result).into())
            };
        }
        Ok (())
    }
    /// adds user to specified match making group in the database for a given 
    /// discord uuid and group name. this is done by calling the add_match_making_user()
    /// stored function.
    ///
    /// the stored function returns the following:
    ///     0: success
    ///     1: failure to add user to database
    ///     2: specified match making group does not exist
    ///     3: user is already registered for this group
    ///
    /// #FIXME: We have to cast the u64 to a string here since the postgres lib can't
    ///         convert a u64 to NUMERIC. Maybe there is a better way to do this.
    ///
    /// # Example
    ///
    /// ```
    /// database::Database::add_mm_user("uuid", "1v1").unwrap();"
    /// ```
    pub async fn add_mm_user (&self, discord_uuid: u64, group: &str) -> Result <i32, Box<dyn Error>> {
        let (client, _) = tokio_postgres::connect(&self.connection_string, NoTls).await?;
        let statement = client.prepare_typed (
            "SELECT add_match_making_user ( $1, $2 );",
            &[Type::TEXT, Type::TEXT]
        ).await?;
        let rows = client.query(&statement, &[&discord_uuid.to_string(), &group]).await?;
        Ok (rows[0].get(0))
    }
    /// removes user from specified match making group in the database for a given 
    /// discord uuid and group name. this is done by calling the remove_match_making_user()
    /// stored function.
    ///
    /// the stored function returns the following:
    ///     0: success
    ///     1: failure to add user to database
    ///     2: specified match making group does not exist
    ///     3: user is not registered for this group
    ///
    /// #FIXME: We have to cast the u64 to a string here since the postgres lib can't
    ///         convert a u64 to NUMERIC. Maybe there is a better way to do this.
    ///
    /// # Example
    ///
    /// ```
    /// database::Database::remove_mm_user("uuid", "1v1").unwrap();"
    /// ```
    pub async fn remove_mm_user (&self, discord_uuid: u64, group: &str) -> Result <i32, Box<dyn Error>> {
        let (client, _) = tokio_postgres::connect(&self.connection_string, NoTls).await?;
        let statement = client.prepare_typed (
            "SELECT remove_match_making_user ( $1, $2 );",
            &[Type::TEXT, Type::TEXT]
        ).await?;
        let rows = client.query(&statement, &[&discord_uuid.to_string(), &group]).await?;
        Ok (rows[0].get(0))
    }

}
