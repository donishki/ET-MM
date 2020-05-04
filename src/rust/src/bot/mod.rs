mod commands;

use crate::config::MMGroup;
use crate::database::Database;
use crate::logger::Log;
use commands:: {
    ping::*,
    subscribe::*,
    unsubscribe::*
};
use serenity:: {
    client::bridge::gateway::ShardManager,
    framework:: {
        StandardFramework,
        standard::macros::group
    },
    model:: {
        channel::ChannelType,
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
    pub fn construct(discord_token: &str, database: &Arc<Database>, log: &Arc<Log>, mm_groups: &Arc<Vec<MMGroup>>) -> Result<Self, Box<dyn Error>> {
        let mut client = Client::new(&discord_token, Handler)?;

        // pack context data
        {
            let mut data = client.data.write();
            data.insert::<Database>(Arc::clone(&database));
            data.insert::<Log>(Arc::clone(&log));
            data.insert::<MMGroup>(Arc::clone(&mm_groups));
            data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        }
        // set owners
        let owners = match client.cache_and_http.http.get_current_application_info() {
            Ok(o) => {
                let mut owners_set = HashSet::new();
                owners_set.insert(o.owner.id);
                owners_set
            },
            Err(e) => return Err(format!("couldn't get application info: \t{}", e).into())
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
                    let log = context.data.read().get::<Log>().cloned().unwrap();
                    error!(log.logger, "\terror in command: {:?}", e;
                        "command" => command,
                        "message" => &message.content,
                        "author"  => &message.author.name
                    );
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
        let log = context.data.read().get::<Log>().cloned().unwrap();
        info!(log.logger, "\t{} connected to discord...", ready.user.name);

        // create match making group channels and roles
        let mm_groups = context.data.read().get::<MMGroup>().cloned().unwrap();
        for (i, guild) in ready.guilds.iter().enumerate() {
            let guild = guild.id();
            info!(log.logger, "\tcreating channels and roles for guild {}...", i);
            //create channels
            let channels = guild.channels(&context.http).unwrap();
            for group in mm_groups.iter() {
                for channel in channels.values() {
                    if channel.kind == ChannelType::Text && channel.name == group.name {
                        info!(log.logger, "\t\tgroup: {} already exists, skipping...", group.name);
                        break;
                    }
                }
                info!(log.logger, "\t\tchannel: {} added to guild: {}...", group.name, i);
                let _ = guild.create_channel(&context.http, |c| c.name(&group.name).kind(ChannelType::Text));
            }
        }
    }
    // handle resume event
    fn resume(&self, context: Context, _: ResumedEvent) {
        let log = context.data.read().get::<Log>().cloned().unwrap();
        info!(log.logger, "\tresumed...");
    }
}

// General structure for bot framework
#[group]
#[commands(ping, subscribe, unsubscribe)]
struct General;

// ShardManagerContainer for bot framework
struct ShardManagerContainer;

// TypeMapKey implementation for ShardManagerContainer
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

// TypeMapKey implementation for Database
impl TypeMapKey for Database {
    type Value = Arc<Database>;
}

// TypeMapKey implementation for Log
impl TypeMapKey for Log {
    type Value = Arc<Log>;
}

// TypeMapKey implementation for MMGroup
impl TypeMapKey for MMGroup {
    type Value = Arc<Vec<MMGroup>>;
}