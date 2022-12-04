pub fn setuphelpfn() -> String {
    let setuphelpstring = r#"
use crate::Error;
use poise::serenity_prelude::{self as serenit, ChannelId};
use serenity::utils::Colour;

/// About command
#[poise::command(slash_command)]
pub async fn help(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    ctx.send(|b| {
        b.embed(|b| b.description(
            "This bot is an example bot and sings a beautiful song for you!
            beep boop beep boop lorem ipsum lalalala"
            ).title("help").colour(Colour::BLITZ_BLUE))
            .ephemeral(true)
            .components(|b| {
                b.create_action_row(|b| {
                    b.create_button(|b| {
                        b.label("Discord.com")
                            .url("https://discord.com/")
                            //.custom_id(1)
                            .style(serenit::ButtonStyle::Link)                        
                    })                                   
                })
            })          
    })
    .await?;
    //When the message is sent in your private channel, return the option to deregister the bot.
    if ctx.channel_id() == ChannelId(/*your channel id*/) {
        poise::builtins::register_application_commands_buttons(ctx).await?;
    }
    Ok(())
}
    


"#;
    return setuphelpstring.to_string();
}