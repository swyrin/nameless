use crate::nameless_types::{NamelessContext, NamelessError};
use crate::persistence::model::honeypot::{Honeypot, HoneypotUpdate};
use crate::persistence::repository::honeypot::{
    delete_honeypot_entry, get_honeypot_entry, set_honeypot_entry,
};
use poise::serenity_prelude;
use poise::serenity_prelude::{ChannelId, Mentionable};
use std::str::FromStr;

/// Honeypot commands.
#[poise::command(
    guild_only,
    slash_command,
    subcommands("get", "set", "unset", "toggle"),
    subcommand_required,
    default_member_permissions = "MANAGE_GUILD",
    required_bot_permissions = "BAN_MEMBERS"
)]
pub async fn honeypot(_: NamelessContext<'_>) -> Result<(), NamelessError> {
    Ok(())
}

/// Get the bound honeypot channel & its status.
#[poise::command(slash_command)]
pub async fn get(ctx: NamelessContext<'_>) -> Result<(), NamelessError> {
    let gid = ctx.guild_id().unwrap();
    let mut db = &ctx.data().db;

    let entry = get_honeypot_entry(gid, &mut db).await;

    match entry {
        Some(Honeypot {
            guild_id: _,
            channel_id,
            enabled,
        }) => {
            let chn = ChannelId::from_str(channel_id.as_str())?;

            let status = match enabled {
                true => "is",
                false => "IS NOT",
            };

            ctx.say(format!(
                "The bounded honeypot channel is {chn}, and {act} watching for messages.",
                chn = chn.mention(),
                act = status
            ))
            .await?
        }
        None => ctx.say("Nothing is bound").await?,
    };

    Ok(())
}

/// Bind a honeypot channel. This will reset the monitoring status.
#[poise::command(slash_command)]
pub async fn set(
    ctx: NamelessContext<'_>,
    #[description = "Honeypot channel to bind."]
    #[channel_types("Text")]
    channel: serenity_prelude::GuildChannel,
) -> Result<(), NamelessError> {
    let mut db = &ctx.data().db;

    set_honeypot_entry(
        HoneypotUpdate {
            guild_id: ctx.guild_id().unwrap().to_string(),
            channel_id: channel.id.to_string(),
            enabled: true,
        },
        &mut db,
    )
    .await;

    ctx.say(format!(
        "Successfully bound honeypot channel to {}",
        channel.mention()
    ))
    .await?;

    channel
        .say(
            &ctx,
            "This channel has been bound as the guild's honeypot channel.",
        )
        .await?;

    Ok(())
}

/// Unbind a honeypot channel.
#[poise::command(slash_command)]
pub async fn unset(ctx: NamelessContext<'_>) -> Result<(), NamelessError> {
    let gid = ctx.guild_id().unwrap();
    let mut db = &ctx.data().db;

    delete_honeypot_entry(gid, &mut db).await;

    ctx.say("Done!").await?;

    Ok(())
}

/// Toggle honeypot monitoring status.
#[poise::command(slash_command)]
pub async fn toggle(ctx: NamelessContext<'_>) -> Result<(), NamelessError> {
    let mut db = &ctx.data().db;
    let entry = get_honeypot_entry(ctx.guild_id().unwrap(), &mut db).await;

    match entry {
        Some(Honeypot {
            guild_id,
            channel_id,
            enabled,
        }) => {
            let new_enablement_state = !enabled;

            set_honeypot_entry(
                HoneypotUpdate {
                    guild_id,
                    channel_id: channel_id.clone(),
                    enabled: new_enablement_state,
                },
                &mut db,
            )
            .await;

            let status = match new_enablement_state {
                true => "is",
                false => "IS NOT",
            };

            let channel = ChannelId::from_str(channel_id.as_str())?;

            ctx.say(format!(
                "Now {chn} {act} watching for messages.",
                chn = channel.mention(),
                act = status
            ))
            .await?;

            let enablement = match new_enablement_state {
                true => "ENABLED",
                false => "DISABLED",
            };

            channel
                .say(
                    &ctx,
                    format!("This channel's honeypot monitoring is now {enablement}."),
                )
                .await?;
        }
        None => {
            ctx.say("This guild has no bound honeypot channel!").await?;
        }
    }

    Ok(())
}
