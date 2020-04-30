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
// respond to ping commands with "pong!"
pub fn ping(context: &mut Context, message: &Message, _: Args) -> CommandResult {
    let log = match context.data.read().get::<Log>().cloned() {
        Some(l) => l,
        None => panic!()
    };
    info!(log.logger, "\texecuting ping function...");
    let _ = message.channel_id.say(&context.http, "pong!");
    Ok(())
}
