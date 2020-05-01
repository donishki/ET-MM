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
// subscribe user calling command to the specified match making group
pub fn subscribe(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let database = context.data.read().get::<Database>().cloned().unwrap();
    let reply;
    if args.len() != 1 {
        // FIXME: print builtin help for command here.
        return Err(CommandError::from("invalid number of arguments."));
    }
    let group = match args.current() {
        Some(a) => a,
        None => {
            reply = format!("");
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
