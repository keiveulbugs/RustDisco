use crate::Error;
use poise::serenity_prelude::{self as serenit};
use serenity::utils::Colour;




// Struct for determining visibility of message later on.
#[derive(poise::ChoiceParameter, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Visibility {
    Private,
    Private_with_dm,
    Public,
    Public_with_dm,
}



#[poise::command(slash_command)]
pub async fn examplecommand (
    ctx: poise::Context<'_, (), Error>,
    // age and the other components can not include capitals, otherwise Discord will throw an error
    #[description = "Your age"] age: u64,
    #[description = "Your name"] name: String,
    #[description = "Visibility of the message"] visibility: Option<Visibility>,
) -> Result<(), Error> {

    // Creates a boolean for deciding if a message should be Epheremal. The defualt is True.
    let visibilityepheremal = match &visibility.unwrap_or(Visibility::Private) {
        Visibility::Private => true,
        Visibility::Public => false,
        Visibility::Private_with_dm => true,
        Visibility::Public_with_dm => false,
    };

    let reply = ctx.send(|b| {
        b.embed(|b| {
            b.description("*Showcasing how the bot can edit a message*")
            .title("wow".to_string())
        })
        .ephemeral(visibilityepheremal)
    })
    .await?;

    
    // Visibility of messages, for testing, you probably want to set the default to "public" without the dm's as it is more test friendly. Personal preference tho.
    let dmbool = match visibility.unwrap_or(Visibility::Private_with_dm) {
        Visibility::Private => false,
        Visibility::Public => false,
        Visibility::Private_with_dm => true,
        Visibility::Public_with_dm => true,
    };



    let mut n :u64 = 0;
    while n < 10000 {
        n += 1;
        //println!("{n}");
        if n%1000 == 0 {
            reply
                .edit(ctx, |b| {
                    b.embed(|b| {
                        b.field(
                            "Starting point",
                            "0",
                            true,
                        )
                        .field(
                            "Last updated amount",
                            format!("{}", n),
                            true,
                        )
                        .field("Goal", "10.000", false)
                        .description(format!("This description suggests that you are {} years old!", age))
                        .title(format!(
                            "Hello {}", name)
                            )
                        .colour(Colour::from_rgb(253, 233, 55))
                        .footer(|b| b.text("A footer warning"))
                    })
                    .ephemeral(visibilityepheremal)
                })
                .await?;
                
        }
    }

    reply.edit(ctx, |b| {
        b.embed(|b| {
            b.description("The end! We counted to 10.000!")
        })
        .components(|b| {
            b.create_action_row(|b| {
                b.create_button(|b| {
                    b.label("A link to cool memes")
                        .url("https://xkcd.com/")
                        .style(serenit::ButtonStyle::Link)
                });
                b.create_button(|b| {
                    b.label("Reactive button")
                        .custom_id("firstbutton")
                        .style(serenit::ButtonStyle::Primary)
                })
            })
        })
    }).await?;
    // Send DM to author if they request it.
    if dmbool {
        ctx.author()
            .dm(ctx.discord(), |f| f.content("You requested a DM!"))
            .await?;
    };

    // Interaction button
    while let Some(mci) = serenit::CollectComponentInteraction::new(ctx.discord())
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(600)) //timeout after 10 minutes
        .filter(move |mci| mci.data.custom_id == "firstbutton".to_string())
        .await
    {
        ctx.send(|b| {
            b.embed(|b| {
                b.description("Sustainable development is development that meets the needs of the present without compromising the ability of future generations to meet their own needs ~Brundtland report, 1987.")
                    .colour(Colour::from_rgb(253, 233, 55))
            })
            .ephemeral(visibilityepheremal)
        })
        .await?;
        mci.create_interaction_response(ctx.discord(), |ir| {
            ir.kind(serenit::InteractionResponseType::DeferredUpdateMessage)
        })
        .await?;
    }

    Ok(())
}