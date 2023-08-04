use crate::Error;
use poise::serenity_prelude::{self as serenit, ChannelId};
use serenity::utils::Colour;

// Default command
#[poise::command(slash_command)]
pub async fn examplecommand(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    ctx.send(|a| {a.content{"Hello world!"}}).await?;
    Ok(())
}