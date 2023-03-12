use crate::Error;
use poise::serenity_prelude::{self as serenit, ChannelId};
use serenity::utils::Colour;

/// About command
#[poise::command(slash_command)]
pub async fn examplecommand(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}