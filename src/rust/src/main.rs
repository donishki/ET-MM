extern crate postgres;
extern crate serenity;
#[macro_use]
extern crate slog;

mod bot;
mod config;
mod database;
#[macro_use]
mod logger;

use std::sync::Arc;

fn main() {
    // initialize logger
    let log = Arc::new(logger::Log::new());

    // FIXME: eventually this will be where arguements are processed
    //        for now just hardcode these parameters
    let bot_config_path: &'static str = "/opt/et-mm-bot/config.cfg";

    // initialize bot
	info!(log.logger, "ET-MM Bot version {}", env!("CARGO_PKG_VERSION"));

    //load bot configuration
    info!(log.logger, "loading configuration into memory...");
    let config = match config::Config::construct(bot_config_path) {
        Ok (b) => b,
        Err(e) => {
            error!(log.logger, "\t{}", e; "file" => bot_config_path);
            drop(log);
            panic!();
        }
    };
    
    // initialize database object
    info!(log.logger, "initializing database object...");
    let db = match database::Database::construct(&config.db_connection_string, &log) {
        Ok (d) => d,
        Err(e) => {
            error!(log.logger, "\t{}", e; "connection string" => config.db_connection_string);
            drop(log);
            panic!();
        }
    };

    // add match making groups to database
    info!(log.logger, "adding configured match making groups...");
    match db.add_mm_groups(&config.mm_groups) {
        Ok (_) => (),
        Err(e) => {
            error!(log.logger, "\t{}", e);
            drop(log);
            panic!();
        }
    };

    // initialize bot
    info!(log.logger, "initializing discord bot...");
    let mut bot = match bot::Bot::construct(&config.discord_token, &log) {
        Ok (b) => b,
        Err(e) => {
            error!(log.logger, "\t{}", e);
            drop(log);
            panic!();
        } 
    };

    // start bot
    info!(log.logger, "starting discord bot...");
    match bot.client.start() {
        Ok (_) => (),
        Err(e) => {
            error!(log.logger, "\t{}", e);
            drop(log);
            panic!();
        }
    };
}
