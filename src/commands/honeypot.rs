use crate::nameless_types::{NamelessContext, NamelessError};
use crate::persistence::services::honeypot::{get_honeypot_channel_id, set_honeypot_channel_id};
use poise::serenity_prelude;
use poise::serenity_prelude::{ChannelId, Mentionable};
use std::str::FromStr;

/// Honeypot commands.
#[poise::command(slash_command, subcommands("get", "set", "unset"), subcommand_required)]
pub async fn honeypot(_: NamelessContext<'_>) -> Result<(), NamelessError> {
    Ok(())
}

/// Get the bound honeypot channel.
#[poise::command(slash_command)]
pub async fn get(ctx: NamelessContext<'_>) -> Result<(), NamelessError> {
    let gid = ctx.guild_id().unwrap();
    let mut db = &ctx.data().db;

    let channel_id = get_honeypot_channel_id(gid, &mut db).await;

    match channel_id {
        Some(x) => {
            let chn = ChannelId::from_str(x.as_str())?;
            let response = format!("The bounded channel is {chn}", chn = chn.mention());

            ctx.say(response).await?
        }
        None => ctx.say("Nothing is bound").await?,
    };

    Ok(())
}

/// Bind a honeypot channel.
#[poise::command(slash_command)]
pub async fn set(
    ctx: NamelessContext<'_>,
    #[description = "Honeypot channel to bind."]
    #[channel_types("Text")]
    channel: serenity_prelude::GuildChannel,
) -> Result<(), NamelessError> {
    let gid = ctx.guild_id().unwrap();
    let mut db = &ctx.data().db;

    set_honeypot_channel_id(gid, Some(channel.id), &mut db).await;

    let resp = format!(
        "Successfully bound honeypot channel to {}",
        channel.mention()
    );

    ctx.say(resp).await?;

    Ok(())
}

/// Unbind a honeypot channel.
#[poise::command(slash_command)]
pub async fn unset(ctx: NamelessContext<'_>) -> Result<(), NamelessError> {
    let gid = ctx.guild_id().unwrap();
    let mut db = &ctx.data().db;

    set_honeypot_channel_id(gid, None, &mut db).await;

    ctx.say("Done!").await?;

    Ok(())
}
