//use crate::logger::Log;

use postgres:: {
    Client,
    NoTls
};
use std::error::Error;

/// Database structure
///
/// # Members
///
///     ```
///     client: database client object
///     ```
pub struct Database {
    client: Client
}

// Database implmentation
impl Database {
    /// connects to postgresql and constructs the database object.
    ///
    /// # Example
    ///
    /// ```
    /// let db = database::Database::construct("host=localhost user=user).unwrap();"
    /// ```
    pub fn construct (connection_string: &str) -> Result<Self, Box<dyn Error>> {
        let mut client = match Client::connect(connection_string, NoTls) {
            Ok (c) => c,
            Err(_) => return Err(source_log!("error connecting to database."))?
        };
        Ok (
            Self {
                client: client
            }
        )
    }
}
