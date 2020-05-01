use crate::database::Database;
use serenity:: {
    framework::standard:: {
        Args,
        CommandResult,
        CommandError,
        macros::command
    },
    model::prelude::*,
    prelude::*
};

#[command]
// subscribe the user calling this function to the match making group matching the name
// of the channel that this function was called from
pub fn subscribe(context: &mut Context, message: &Message, _: Args) -> CommandResult {
    let database = context.data.read().get::<Database>().cloned().unwrap();
    let reply;
    let group = match message.channel_id.name(&context) {
        Some(g) => g,
        None => {
            reply = format!("error retrieving channel name.");
            let _ = message.channel_id.say(&context.http, &reply);
            return Err(CommandError::from(reply));
        }
    };
    let result = match database.add_mm_user(*message.author.id.as_u64(), &group) {
        Ok (r) => r,
        Err(e) => {
            reply = format!("database error: {}", e);
            let _ = message.channel_id.say(&context.http, &reply);
            return Err(CommandError::from(reply))
        }
    };
    match result {
        0 => {
            reply = format!("`{}` has been subscribed to the `{}` match making group.", message.author.name, group);
            let _ = message.channel_id.say(&context.http, &reply);
            return Ok(());
        },
        1 => reply = format!("failed to add `{}` to the database.", message.author.name),
        2 => reply = format!("match making group: `{}` does not exist.", group),
        3 => reply = format!("`{}` is already registered to match making group: `{}`", message.author.name, group),
        _ => reply = format!("database returned an unkown result when calling `add_match_making_user()`: `{}`", result)
    };
    let _ = message.channel_id.say(&context.http, &reply);
    Err(CommandError::from(reply))
}
