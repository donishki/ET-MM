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
    let connection_string: &'static str = "host=localhost user=et-mm";
    let db = match database::Database::construct(connection_string) {
        Ok (d) => d,
        Err(e) => {
            error!(log.logger, "\t{}", e; "connection string" => connection_string);
            drop(log);
            panic!();
        }
    };
}
