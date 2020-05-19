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
// unsubscribe the user calling this function to the match making group matching the name
// of the channel that this function was called from
pub async fn unsubscribe(context: &Context, message: &Message, _: Args) -> CommandResult {
    let reply;
    // retrieve matchmaking group
    let group = match message.channel_id.name(&context).await {
        Some(g) => g,
        None => {
            reply = "error retrieving channel name.".to_string();
            let _ = message.channel_id.say(&context.http, &reply);
            return Err(CommandError::from(reply))
        }
    };
    // retrieve guild
    let guild = match message.guild(&context).await {
        Some(g) => g,
        None => {
            reply = "error retrieving guild information.".to_string();
            let _ = message.channel_id.say(&context.http, &reply);
            return Err(CommandError::from(reply))
        }
    };
    // retrieve member
    let mut member = match guild.member(&context.http, &message.author.id).await {
        Ok (m) => m,
        Err(e) => {
            reply = format!("error retrieving member: `{}`.", e);
            let _ = message.channel_id.say(&context.http, &reply);
            return Err(CommandError::from(reply))
        }
    };
    // retrieve role
    let role = match guild.role_by_name(&group) {
        Some(r) => r,
        None => {
            reply = format!("error retrieving role `{}`.", &group);
            let _ = message.channel_id.say(&context.http, &reply);
            return Err(CommandError::from(reply))
        }
    };
    // remove role from discord and database
    let result = {
        // remove role from discord
        if let Err(e) = member.remove_role(&context.http, role).await {
            reply = format!("error removing role: `{}` for user: `{}` error: `{}`", role, message.author.name, e);
            let _ = message.channel_id.say(&context.http, &reply);
            return Err(CommandError::from(reply))            
        }
        // retrieve database
        let database = match context.data.read().await.get::<Database>().cloned() {
            Some(d) => d,
            None => {
                reply = "error retrieving database object".to_string();
                let _ = message.channel_id.say(&context.http, &reply);
                return Err(CommandError::from(reply))                
            }
        };
        // remove role from database
        let database = database.read().await;
        match database.remove_mm_user(*message.author.id.as_u64(), &group).await {
            Ok (r) => r,
            Err(e) => {
                reply = format!("database error: {}", e);
                let _ = message.channel_id.say(&context.http, &reply);
                return Err(CommandError::from(reply))
            }
        }
    };
    // return
    match result {
        0 => reply = format!("`{}` has been unsubscribed from the `{}` match making group.", message.author.name, group),
        1 => reply = format!("failed to add `{}` to the database.", message.author.name),
        2 => reply = format!("match making group: `{}` does not exist.", group),
        3 => reply = format!("`{}` is not subscribed to match making group: `{}`", message.author.name, group),
        _ => reply = format!("database returned an unkown result when calling `remove_match_making_user()`: `{}`", result)
    };
    let _ = message.channel_id.say(&context.http, &reply).await?;
    Ok (())
}
