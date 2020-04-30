use crate::database::Database;
use crate::logger::Log;
use serenity:: {
    framework::standard:: {
        Args,
        CommandResult,
        macros::command
    },
    model::prelude::*,
    prelude::*
};

#[command]
// subscribe user calling command to the specified match making group
pub fn subscribe(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let log = match context.data.read().get::<Log>().cloned() {
        Some(l) => l,
        None => panic!()
    };
    info!(log.logger, "\tprocessing subscribe command"; "user" => &message.author.name);
    let database = match context.data.read().get::<Database>().cloned() {
        Some(d) => d,
        None => {
            error!(log.logger, "\t\tfailed to invoke database object");
            return Ok(());
        }
    };
    for arg in args.iter::<String>() {
        let arg = arg.unwrap();
        let result = match database.add_mm_user(message.author.id.as_u64(), &arg) {
            Ok (r) => r,
            Err(e) => {
                error!(log.logger, "\t\t{}", e);
                return Ok(());
            }
        };
        match result {
            0 => info!(log.logger, "\t\tadded user to group"; "group" => arg),
            1 => warn!(log.logger, "\t\tfailed to add user to database"),
            2 => warn!(log.logger, "\t\tuser already belongs to group"; "group" => arg),
            _ => {
                error!(log.logger, "unknown database result for add_match_making_user function: {}", result);
                return Ok(());
            }
        };
    }

    // let _ = message.channel_id.say(&context.http, "pong!");
    Ok(())
}
