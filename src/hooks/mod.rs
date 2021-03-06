use serenity::{
    framework::standard::{macros::hook, CommandError},
    model::channel::Message,
    prelude::*,
};
use tracing::info;

pub mod roles;

#[hook]
pub async fn before(_: &Context, msg: &Message, command_name: &str) -> bool {
    info!(
        "Got command '{}' from user '{}' ({})",
        command_name, msg.author.name, msg.author.id.0
    );

    true
}

#[hook]
pub async fn after(_: &Context, msg: &Message, cmd_name: &str, error: Result<(), CommandError>) {
    if let Err(why) = error {
        info!("Error in {} by {}: {:?}", cmd_name, msg.author.name, why);
    } else {
        info!(
            "Finished executing command '{}' by user '{}'",
            cmd_name, msg.author.name
        );
    }
}
