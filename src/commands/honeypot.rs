use std::str::FromStr;

use poise::serenity_prelude;
use poise::serenity_prelude::{ChannelId, Mentionable};
use sea_orm::Set;

use crate::persistence::model::guild;
use crate::persistence::model::guild::GuildHoneypotUpdatePayload;
use crate::persistence::repository::guild::{get_guild, insert_guild, update_guild};
use crate::types::{CommandContext, CommandError};

/// Honeypot commands.
#[poise::command(
    guild_only,
    slash_command,
    subcommands("get", "set", "unset"),
    subcommand_required,
    default_member_permissions = "MANAGE_GUILD",
    required_bot_permissions = "BAN_MEMBERS"
)]
#[allow(clippy::unused_async, reason = "Async required by poise.")]
pub async fn honeypot(_: CommandContext<'_>) -> Result<(), CommandError> {
    Ok(())
}

/// Get the bound honeypot channel.
#[poise::command(slash_command)]
pub async fn get(ctx: CommandContext<'_>) -> Result<(), CommandError> {
    let gid = ctx.guild_id().unwrap();
    let db = ctx.data().db.clone();

    if let Some(record) = get_guild(gid, &db).await
        && let Some(honeypot_chn) = record.honeypot_channel
    {
        let channel_id = ChannelId::from_str(&honeypot_chn)?;

        ctx.say(format!("The bounded honeypot channel is {chn}.", chn = channel_id.mention()))
            .await?;
    } else {
        ctx.say("Nothing is bound").await?;
    }

    Ok(())
}

/// Bind a honeypot channel.
#[poise::command(slash_command)]
pub async fn set(
    ctx: CommandContext<'_>,
    #[description = "Honeypot channel to bind."]
    #[channel_types("Text")]
    channel: serenity_prelude::GuildChannel,
) -> Result<(), CommandError> {
    let gid = ctx.guild_id().unwrap();
    let db = ctx.data().db.clone();

    if get_guild(gid, &db).await.is_some() {
        update_guild(
            gid,
            GuildHoneypotUpdatePayload {
                honeypot_channel: Some(Some(channel.id.to_string())),
            },
            &db,
        )
        .await;
    } else {
        insert_guild(
            guild::ActiveModel {
                id: Set(gid.to_string()),
                honeypot_channel: Set(Some(channel.id.to_string())),
            },
            &db,
        )
        .await;
    }

    ctx.say(format!("Successfully bound honeypot channel to {}", channel.mention())).await?;

    Ok(())
}

/// Unbind a honeypot channel.
#[poise::command(slash_command)]
pub async fn unset(ctx: CommandContext<'_>) -> Result<(), CommandError> {
    let gid = ctx.guild_id().unwrap();
    let db = ctx.data().db.clone();

    if get_guild(gid, &db).await.is_some() {
        update_guild(
            gid,
            GuildHoneypotUpdatePayload {
                honeypot_channel: Some(None),
            },
            &db,
        )
        .await;

        ctx.say("This guild no longer has honeypot channel.").await?;
    } else {
        ctx.say("This guild has no bounded honeypot channel.").await?;
    }

    Ok(())
}
