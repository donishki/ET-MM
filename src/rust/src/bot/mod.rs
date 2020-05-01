mod commands;

use commands:: {
    ping::*,
    subscribe::*
};
use crate::database::Database;
use crate::logger::Log;
use serenity:: {
    client::bridge::gateway::ShardManager,
    framework:: {
        StandardFramework,
        standard::macros::group
    },
    model:: {
        event::ResumedEvent,
        gateway::Ready
    },
    prelude::*
};
use std:: {
    collections::HashSet,
    error::Error,
    sync::Arc
};

/// Bot structure for discord bot
///
/// # Members
///
///     ```
///     client: serenity discord client
///     ```
pub struct Bot {
    pub client: Client
}

// Bot implementation
impl Bot {
    /// constructs the serenity discord client object
    ///
    /// # Example
    ///
    /// ```
    /// let log = Arc::new(logger::Log::new());
    /// let discord_token = "token";
    /// let mut bot = bot::Bot::construct(&discord_token, &log).unwrap();
    /// ```
    pub fn construct(discord_token: &str, database: &Arc<Database>, log: &Arc<Log>) -> Result<Self, Box<dyn Error>> {
        let mut client = Client::new(&discord_token, Handler)?;

        // pack context data
        {
            let mut data = client.data.write();
            data.insert::<Database>(Arc::clone(&database));
            data.insert::<Log>(Arc::clone(&log));
            data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        }
        // set owners
        let owners = match client.cache_and_http.http.get_current_application_info() {
            Ok(o) => {
                let mut owners_set = HashSet::new();
                owners_set.insert(o.owner.id);
                owners_set
            },
            Err(e) => return Err(format!("couldn't get application info: {}", e).into())
        };
        // initialize framework
        client.with_framework(StandardFramework::new()
            .configure(|c| c
                .owners(owners)
                .prefix("!")
            )
            .group(&GENERAL_GROUP)
            // handle command errors
            .after(|context, message, command, result| {
                if let Err(e) = result {
                    let log = match context.data.read().get::<Log>().cloned() {
                        Some(l) => l,
                        None => panic!()
                    };
                    error!(log.logger, "\terror in command: {:?}", e; "command" => command);
                    error!(log.logger, "\tcalled by message: {:?}", message);
                }
            })
        );
        Ok (
            Self {
                client
            }
        )
    }
}

// Handler structure
struct Handler;

// EventHandler implementation for Handler
impl EventHandler for Handler {
    // handle ready event
    fn ready(&self, context: Context, ready: Ready) {
        let log = match context.data.read().get::<Log>().cloned() {
            Some(l) => l,
            None => panic!()
        };
        info!(log.logger, "\t{} connected to discord...", ready.user.name);
    }
    // handle resume event
    fn resume(&self, context: Context, _: ResumedEvent) {
        let log = match context.data.read().get::<Log>().cloned() {
            Some(l) => l,
            None => panic!()
        };
        info!(log.logger, "\tresumed...");
    }
}

// General structure for bot framework
#[group]
#[commands(ping, subscribe)]
struct General;

// ShardManagerContainer for bot framework
struct ShardManagerContainer;

// TypeMapKey implementation for Database
impl TypeMapKey for Database {
    type Value = Arc<Database>;
}

// TypeMapKey implementation for ShardManagerContainer
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

// TypeMapKey implementation for Log
impl TypeMapKey for Log {
    type Value = Arc<Log>;
}
