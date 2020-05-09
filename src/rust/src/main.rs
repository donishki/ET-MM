extern crate postgres;
extern crate serenity;
#[macro_use]
extern crate slog;

mod bot;
mod config;
mod database;
#[macro_use]
mod logger;

use std::sync:: {
    Arc,
    RwLock
};

#[tokio::main]
async fn main() {
    // initialize logger
    let log_lock = Arc::new(RwLock::new(logger::Log::new()));
    // FIXME: eventually this will be where arguements are processed
    //        for now just hardcode these parameters
    // initialize bot
    {
        let log = log_lock.read().unwrap();
        info!(log.logger, "ET-MM Bot version {}", env!("CARGO_PKG_VERSION"));
    }
    // load bot configuration
    let bot_config_path: &'static str = "/opt/et-mm-bot/config.cfg";
    let config = {
        let log = log_lock.read().unwrap();
        info!(log.logger, "loading configuration into memory...");
        match config::Config::construct(bot_config_path) {
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
        let log = log_lock.read().unwrap();
        info!(log.logger, "initializing database object...");
        match database::Database::construct(&config, &log_lock) {
            Ok (d) => Arc::new(RwLock::new(d)),
            Err(e) => {
                error!(log.logger, "\t{}", e; "connection string" => config.database_connection_string);
                drop(log);
                drop(log_lock);
                panic!();
            }
        }
    };
    // add match making groups to database
    {
        let log = log_lock.read().unwrap();
        let database = database_lock.read().unwrap();
        info!(log.logger, "adding configured match making groups...");
        match database.add_mm_groups(&config.mm_groups) {
            Ok (_) => (),
            Err(e) => {
                error!(log.logger, "\t{}", e);
                drop(log);
                drop(log_lock);
                panic!();
            }
        };
    }
    // // initialize discord bot
    // let mut bot = {
    //     {
    //         let log = log_lock.read().unwrap();
    //         info!(log.logger, "initializing discord bot...");
    //     }
    //     match bot::Bot::construct(&config, &database_lock, &log_lock).await {
    //         Ok (b) => b,
    //         Err(e) => {
    //             let log = log_lock.read().unwrap();
    //             error!(log.logger, "\t{}", e);
    //             drop(log);
    //             drop(log_lock);
    //             panic!();
    //         }
    //     }
    // };
    // // start bot
    // {
    //     let log = log_lock.read().unwrap();
    //     info!(log.logger, "starting discord bot...");
    //     if let Err(e) = bot.start().await {
    //         error!(log.logger, "\t{}", e);
    //         drop(log);
    //         panic!();
    //     };
    // }
}
