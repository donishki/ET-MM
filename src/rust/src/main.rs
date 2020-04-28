#[macro_use]
extern crate slog;
extern crate postgres;

mod database;
mod bot;
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
    let mut db = match database::Database::construct(&bot.config.db_connection_string) {
        Ok (d) => d,
        Err(e) => {
            error!(log.logger, "\t{}", e; "connection string" => bot.config.db_connection_string);
            drop(log);
            panic!();
        }
    };

    // add match making groups to database
    info!(log.logger, "\tadding configured match making groups...");
    for group in bot.config.mm_groups.iter() {
        let result: i32 = match db.add_mm_group(group) {
            Ok (r) => r,
            Err(e) => {
                error!(log.logger, "\t{}", e; "group" => "1v1");
                drop(log);
                panic!();
            }
        };
        if result != 0 {
            warn!(log.logger, "\tgroup already exists in database"; "group" => group);
        } else {
            info!(log.logger, "\tadded group"; "group" => group);
        }
    }
}
