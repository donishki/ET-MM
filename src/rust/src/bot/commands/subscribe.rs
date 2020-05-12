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
pub async fn subscribe(context: &Context, message: &Message, _: Args) -> CommandResult {
    // retrieve match making group
    let reply;
    let group = match message.channel_id.name(&context).await {
        Some(g) => g,
        None => {
            reply = "error retrieving channel name.".to_string();
            let _ = message.channel_id.say(&context.http, &reply);
            return Err(CommandError::from(reply));
        }
    };
    // execute database function
    let result = {
        // retrieve database
        let database_lock = context.data.read().await.get::<Database>().cloned().unwrap();
        let database = database_lock.read().await;
        match database.add_mm_user(*message.author.id.as_u64(), &group).await {
            Ok (r) => r,
            Err(e) => {
                reply = format!("{}", e);
                let _ = message.channel_id.say(&context.http, &reply);
                return Err(CommandError::from(reply))
            }
        }
    };
    // return
    match result {
        0 => reply = format!("`{}` has been subscribed to the `{}` match making group.", message.author.name, group),
        1 => reply = format!("failed to add `{}` to the database.", message.author.name),
        2 => reply = format!("match making group: `{}` does not exist.", group),
        3 => reply = format!("`{}` is already subscribed to match making group: `{}`", message.author.name, group),
        _ => reply = format!("database returned an unkown result when calling `add_match_making_user()`: `{}`", result)
    };
    let _ = message.channel_id.say(&context.http, &reply).await?;
    Ok (())
}
