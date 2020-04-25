#[macro_use]
extern crate slog;
extern crate postgres;

#[macro_use]
mod logger;
mod database;

fn main() {
    // initialize logger
    let log = logger::Log::new();

    // initialize bot
	info!(log.logger, "ET-MM Bot version {}", env!("CARGO_PKG_VERSION"));
    
    // initialize database object
    info!(log.logger, "\tinitializing database object...");
    let connection_string: &'static str = "host=localhost user=et_mm";
    let mut db = match database::Database::construct(connection_string) {
        Ok (d) => d,
        Err(e) => {
            error!(log.logger, "\t\t{}", e; "connection string" => connection_string);
            drop(log);
            panic!();
        }
    };

    // add match making groups to database
    // FIXME: these should be parsed from a configuration file
    //        that should be parsed and loaded into memory.
    info!(log.logger, "\tadding configured match making groups...");
    let result: i8 = match db.add_mm_group("1v1") {
        Ok (r) => r,
        Err(e) => {
            error!(log.logger, "\t\t{}", e; "group" => "1v1");
            drop(log);
            panic!();
        }
    };
    if result != 0 {
        warn!(log.logger, "\t\tgroup already exists in database"; "group" => "1v1")
    };
}
