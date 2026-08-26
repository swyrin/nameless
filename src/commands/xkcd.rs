use crate::nameless_types::{NamelessContext, NamelessError};
use poise::serenity_prelude::Timestamp;
use poise::{
    CreateReply,
    serenity_prelude::{CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter},
};
use xkcd::entry::XkcdEntry;
use xkcd::fetch::asynchronous::{fetch_latest, fetch_number, fetch_random};

fn create_xkcd_button(entry: &XkcdEntry) -> CreateButton {
    CreateButton::new_link(format!("https://xkcd.com/{}/", entry.num)).label("See on XKCD")
}

fn create_xkcd_embed(entry: &XkcdEntry) -> CreateEmbed {
    // mfw XKCD does not expose time.
    let iso8601 = format!(
        "{year}-{month:0>2}-{day:0>2}T00:00:00Z",
        year = entry.year,
        month = entry.month,
        day = entry.day
    );

    CreateEmbed::new()
        .title(format!(
            "{num}: {title}",
            num = entry.num,
            title = entry.safe_title
        ))
        .image(entry.img.clone())
        .timestamp(Timestamp::parse(&iso8601).expect("Malform ISO 8601 format."))
        .footer(CreateEmbedFooter::new(entry.alt.clone()))
}

fn create_xkcd_reply(entry: &XkcdEntry) -> CreateReply {
    let embed = create_xkcd_embed(entry);
    let action_row = CreateActionRow::Buttons(vec![create_xkcd_button(entry)]);

    CreateReply::default()
        .embed(embed)
        .components(vec![action_row])
}

/// XKCD commands.
#[poise::command(slash_command, subcommands("get", "random"), subcommand_required)]
#[allow(clippy::unused_async, reason = "Async required by poise.")]
pub async fn xkcd(_: NamelessContext<'_>) -> Result<(), NamelessError> {
    Ok(())
}

/// Get an XKCD comic.
#[poise::command(slash_command)]
pub async fn get(
    ctx: NamelessContext<'_>,
    #[description = "XKCD comic number, blank for latest."]
    #[min = 1]
    number: Option<u64>,
) -> Result<(), NamelessError> {
    // https://github.com/seanmonstar/reqwest/issues/1017
    let xkcd = match number {
        Some(num) => fetch_number(num).await,
        None => fetch_latest().await,
    }
    .expect("Unable to call XKCD API");

    ctx.send(create_xkcd_reply(&xkcd)).await?;

    Ok(())
}

/// Get a random XKCD comic.
#[poise::command(slash_command)]
pub async fn random(ctx: NamelessContext<'_>) -> Result<(), NamelessError> {
    let xkcd = fetch_random().await.expect("Unable to call XKCD API");

    ctx.send(create_xkcd_reply(&xkcd)).await?;

    Ok(())
}
