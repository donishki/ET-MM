use postgres:: {
    Client,
    NoTls,
    types::Type
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
    /// let db = database::Database::construct("host=localhost user=user").unwrap();"
    /// ```
    pub fn construct (connection_string: &str) -> Result<Self, Box<dyn Error>> {
        let client = Client::connect(connection_string, NoTls)?;
        Ok (
            Self {
                client: client
            }
        )
    }
    /// calls the add_match_making_group database function, the database returns
    /// 0 on success and 1 on failure due to the group existing already.
    ///
    /// # Example
    ///
    /// ```
    /// let result = database::Database::add_mm_group("1v1").unwrap();"
    /// assert_eq!(result, 0)
    /// ```
    pub fn add_mm_group (&mut self, group_name: &str) -> Result <i8, Box<dyn Error>> {
        let statement = self.client.prepare_typed (
            "SELECT add_match_making_group ( '?' );",
            &[Type::TEXT]
        )?;
        let rows = self.client.query(&statement, &[&group_name])?;
        let result: i8 = rows[0].get(0);
        Ok (
            result
        )
    }
}
