#[macro_use]
extern crate slog;
extern crate postgres;

mod bot;
mod database;
#[macro_use]
mod logger;

fn main() {
    // initialize logger
    let log = logger::Log::new();

    // FIXME: eventually this will be where arguements are processed
    //        for now just hardcode these parameters
    let bot_config_path: &'static str = "/opt/et-mm-bot/config.cfg";

    // initialize bot
	info!(log.logger, "ET-MM Bot version {}", env!("CARGO_PKG_VERSION"));

    //load bot configuration
    info!(log.logger, "loading bot configuration into memory...");
    let bot = match bot::Bot::construct(bot_config_path) {
        Ok (b) => b,
        Err(e) => {
            error!(log.logger, "\t{}", e; "file" => bot_config_path);
            drop(log);
            panic!();
        }
    };
    
    // initialize database object
    info!(log.logger, "initializing database object...");
    let db = match database::Database::construct(&bot.config.db_connection_string, &log) {
        Ok (d) => d,
        Err(e) => {
            error!(log.logger, "\t{}", e; "connection string" => bot.config.db_connection_string);
            drop(log);
            panic!();
        }
    };

    // add match making groups to database
    info!(log.logger, "adding configured match making groups...");
    match db.add_mm_groups(&bot.config.mm_groups) {
        Ok (_) => (),
        Err(e) => {
            error!(log.logger, "\t{}", e);
            drop(log);
            panic!();
        }
    };
}
