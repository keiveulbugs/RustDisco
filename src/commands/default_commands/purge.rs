use crate::Error;
use poise::serenity_prelude::{self as serenit, Member};



// This command allows the owner of a guild / server to kick all users with a specific role.
#[poise::command(slash_command, guild_only = true, required_permissions="ADMINISTRATOR", ephemeral=true)]
pub async fn examplecommand(ctx: poise::Context<'_, (), Error>,
#[description = "Which role to purge?"] role: serenity::model::guild::Role,
#[description = "List users?"] list :bool,
#[description = "Reason for kicking"] reason :String
) -> Result<(), Error> {

    let guildid = ctx.guild_id().unwrap();
    let guilding = ctx.guild().unwrap();
    if guilding.owner_id != ctx.author().id {
        ctx.say("You are not an owner on this server").await?;
        
        panic!("You are not an owner on this server")
    };


    ctx.say(format!("Checking users with role: {}", role.name)).await?;
    let vecofusers = serenity::model::id::GuildId::members(guildid, ctx, Some(1000), None).await?;
    let mut purgeusersvec :Vec<Member> = vec![];

    // Iterate over all users found, and check if they have the role specified.
    for users in vecofusers {
        if users.roles.contains(&role.id) {
            if list {
                ctx.say(format!{"{:#?}", users.display_name()}).await?;
            }
            purgeusersvec.push(users.clone());            
        };
    };

    let buttonmsg = ctx.send(|b| {
        b.content(format!("There are {} users to be Kicked", &purgeusersvec.len()))
        .components(|b| {
            b.create_action_row(|b| {
                b.create_button(|b| {
                    b.label("Kick users")
                    .custom_id("userban")
                })
            })
        })
    }).await?;

    while let Some(mci) = serenit::CollectComponentInteraction::new(ctx)
    .author_id(ctx.author().id)
    .channel_id(ctx.channel_id())
    .timeout(std::time::Duration::from_secs(600)) //timeout after 10 minutes
    .filter(move |mci| mci.data.custom_id == "userban".to_string())
    .await
{
    for user in purgeusersvec.clone() {
        match user.kick_with_reason(ctx, reason.as_str()).await {
            Ok(()) => buttonmsg.clone(), // Check if this can be removed.
            _ => ctx.say(format!("Can't kick {}", user)).await?
        };
    }

   
    mci.create_interaction_response(ctx, |ir| {
        ir.kind(serenit::InteractionResponseType::DeferredUpdateMessage)
    })
    .await?;
    buttonmsg.delete(ctx).await?;
    ctx.send(|b| {
        b.content(format!("**TRIED TO KICK {} USERS**", purgeusersvec.len()))
        
    }).await?;
}
    

    Ok(())
}
    


