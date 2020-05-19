mod commands;

use crate::config:: {
    Config,
    MMGroup
};
use crate::database::Database;
use crate::logger::Log;
use commands:: {
    subscribe::*,
    unsubscribe::*
};
use serenity:: {
    async_trait,
    framework:: {
        StandardFramework,
        standard:: {
            CommandResult,
            macros:: {
                group,
                hook
            }
        }
    },
    http::Http,
    model:: {
        channel:: {
            ChannelType,
            Message
        },
        event::ResumedEvent,
        gateway::Ready,
        guild::Guild
    },
    prelude::*
};
use std:: {
    collections::HashSet,
    error::Error,
    sync::Arc
};
use tokio::sync::RwLock;

/// Bot structure for discord bot
///
/// # Members
///
///     ```
///     client: serenity discord client
///     ```
pub struct Bot {
    client: Client
}

// Bot implementation
impl Bot {
    /// constructs the serenity discord client object
    ///
    /// # Example
    ///
    /// ```
    /// let log = Arc::new(RwLock::new(logger::Log::new()));
    /// let config = config::Config::construct("/opt/et-mm-bot/config.cfg").unwrap();
    /// let database = database::Database::construct(&config, &log).await.unwrap();
    /// let mut bot = bot::Bot::construct(&config, &database, &log).await.unwrap();
    /// ```
    pub async fn construct(config: &Config, database: &Arc<RwLock<Database>>, log: &Arc<RwLock<Log>>) -> Result<Self, Box<dyn Error>> {
        // construct owners hash set
        let http = Http::new_with_token(&config.discord_token);
        let (owners, _bot_id) = match http.get_current_application_info().await {
            Ok(o) => {
                let mut set = HashSet::new();
                set.insert(o.owner.id);
                (set, o.id)
            },
            Err(e) => return Err(format!("could not get application info: {}", e).into())
        };
        // construct framework
        let framework  = StandardFramework::new()
            .configure(|c| c
                .owners(owners)
                .prefix("!")
            )
            .group(&GENERAL_GROUP)
            .after(after);
        // construct client
        let client = match Client::new(&config.discord_token)
            .framework(framework)
            .event_handler(Handler).await {
                Ok (c) => c,
                Err(e) => return Err(format!("error building discord client: {}", e).into())
            };
        // pack context data
        {
            let mut data = client.data.write().await;
            data.insert::<Database>(Arc::clone(&database));
            data.insert::<Log>(Arc::clone(&log));
            data.insert::<MMGroup>(Arc::clone(&config.mm_groups));
        }
        // return
        Ok (
            Self {
                client
            }
        )
    }
    /// starts the serenity discord client object
    ///
    /// # Example
    ///
    /// ```
    /// let log = Arc::new(RwLock::new(logger::Log::new()));
    /// let config = config::Config::construct("/opt/et-mm-bot/config.cfg").unwrap();
    /// let database = database::Database::construct(&config, &log).await.unwrap();
    /// let mut bot = bot::Bot::construct(&config, &database, &log).await.unwrap();
    /// let _ = bot.start().await.unwrap();
    /// ```
    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        // start bot
        if let Err(e) = self.client.start().await {
            return Err(format!("could not start bot: \t{}", e).into())
        };        
        Ok (())
    }
}

// General structure for bot framework
#[group]
#[commands(subscribe, unsubscribe)]
struct General;

// Handler structure
struct Handler;

// EventHandler implementation for Handler
#[async_trait]
impl EventHandler for Handler {
    // handle ready event
    async fn ready (&self, context: Context, ready: Ready) {
        // retrieve log
        let log_lock = context.data.read().await.get::<Log>().cloned().unwrap();
        let log = log_lock.read().await;
        // initialize
        info!(log.logger, "\t{} connected to discord...", ready.user.name);
        // retrieve match making groups
        let mm_groups_lock = match context.data.read().await.get::<MMGroup>().cloned() {
            Some(m) => m,
            None => {
                error!(log.logger, "\terror retrieving match making groups lock...");
                drop(log);
                drop(log_lock);
                panic!();
            }
        };
        let mm_groups = mm_groups_lock.read().await;
        // retrieve database
        let database_lock = match context.data.read().await.get::<Database>().cloned() {
            Some(d) => d,
            None => {
                error!(log.logger, "\terror retrieving database lock...");
                drop(log);
                drop(log_lock);
                panic!()
            }
        };
        let database = database_lock.read().await;
        // add match making groups to database
        info!(log.logger, "\tadding match making groups to database...");
        if let Err(e) = database.add_mm_groups(&mm_groups).await {
            error!(log.logger, "\t\t{}", e);
            drop(log);
            drop(log_lock);
            panic!();
        }
        // add match making group channels and roles
        for (i, guild) in ready.guilds.iter().enumerate() {
            let guild = Guild::get(&context.http, guild.id()).await.unwrap();
            info!(log.logger, "\tcreating match making channels and roles for guild: {}", i);
            let channels = guild.channels(&context.http).await.unwrap();
            for group in mm_groups.iter() {
                if channels.values().any(|channel| channel.kind == ChannelType::Text && channel.name == group.name) {
                    warn!(log.logger, "\t\tskipping: already exists"; "channel" => &group.name);
                } else {
                    let _ = guild.create_channel(&context.http, |c| c.name(&group.name).kind(ChannelType::Text));
                    info!(log.logger, "\t\tadded"; "channel" => &group.name); 
                }
                if guild.roles.values().any(|role| role.name == group.name) {
                    warn!(log.logger, "\t\tskipping: already exists"; "role" => &group.name);
                } else {
                    let _ = guild.create_role(&context.http, |role| role.hoist(true).name(&group.name)).await;
                    info!(log.logger, "\t\tadded"; "role" => &group.name);
                }
                continue;
            }
        }
    }
    // handle resume event
    async fn resume(&self, context: Context, _: ResumedEvent) {
        // retrieve log
        let log = context.data.read().await.get::<Log>().cloned().unwrap();
        let log = log.read().await;
        info!(log.logger, "\tresumed...");
    }
}

// TypeMapKey implementation for Database
impl TypeMapKey for Database {
    type Value = Arc<RwLock<Database>>;
}

// TypeMapKey implementation for Log
impl TypeMapKey for Log {
    type Value = Arc<RwLock<Log>>;
}

// TypeMapKey implementation for MMGroup
impl TypeMapKey for MMGroup {
    type Value = Arc<RwLock<Vec<MMGroup>>>;
}

// hooked bot command error handling function
#[hook]
async fn after(context: &Context, message: &Message, command: &str, result: CommandResult) {
    if let Err(e) = result {
        // retrieve log
        let log = context.data.read().await.get::<Log>().cloned().unwrap();
        let log = log.read().await;
        error!(log.logger, "\terror in command: {:?}", e;
            "command" => command,
            "message" => &message.content,
            "author"  => &message.author.name
        );
    }
}
