extern crate serenity;
#[macro_use]
extern crate slog;
extern crate tokio_postgres;

mod bot;
mod config;
mod database;
#[macro_use]
mod logger;

use bot::Bot;
use config::Config;
use database::Database;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    // initialize logger
    let log_lock = Arc::new(RwLock::new(logger::Log::new()));
    let log = log_lock.read().await;
    // FIXME: eventually this will be where arguements are processed
    //        for now just hardcode these parameters
    let bot_config_path: &'static str = "/opt/et-mm-bot/config.cfg";
    // initialize bot
    info!(log.logger, "ET-MM Bot version {}", env!("CARGO_PKG_VERSION"));
    // load bot configuration
    let config = {
        info!(log.logger, "loading configuration into memory...");
        match Config::construct(bot_config_path) {
            Ok (b) => b,
            Err(e) => {
                error!(log.logger, "\t{}", e; "file" => bot_config_path);
                drop(log);
                drop(log_lock);
                panic!();
            }
        }
    };
    // initialize database object
    let database_lock = {
        info!(log.logger, "initializing database object...");
        match Database::construct(&config, &log_lock).await {
            Ok (d) => Arc::new(RwLock::new(d)),
            Err(e) => {
                error!(log.logger, "\t{}", e; "connection string" => config.database_connection_string);
                drop(log);
                drop(log_lock);
                panic!();
            }
        }
    };
    // initialize discord bot
    let mut bot = {
        info!(log.logger, "initializing discord bot...");
        match Bot::construct(&config, &database_lock, &log_lock).await {
            Ok (b) => b,
            Err(e) => {
                error!(log.logger, "\t{}", e);
                drop(log);
                drop(log_lock);
                panic!();
            }
        }
    };
    // start bot
    info!(log.logger, "starting discord bot...");
    if let Err(e) = bot.start().await {
        error!(log.logger, "\t{}", e);
        drop(log);
        panic!();
    };
}
