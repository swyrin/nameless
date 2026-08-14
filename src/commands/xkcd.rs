use jiff::Timestamp;
use poise::{
    CreateReply,
    serenity_prelude::{CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter},
};
use xkcd_nameless::entry::XkcdEntry;

use crate::nameless_types::{NamelessContext, NamelessError};

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

    let publishing_date: Timestamp = iso8601.parse().unwrap();
    let epoch = publishing_date.as_second();

    CreateEmbed::new()
        .title(format!(
            "{num}: {title}",
            num = entry.num,
            title = entry.safe_title
        ))
        .description(format!("Published: <t:{epoch}:d> (<t:{epoch}:R>)"))
        .image(entry.img.clone())
        .footer(CreateEmbedFooter::new(entry.alt.clone()))
}

fn create_xkcd_reply(entry: XkcdEntry) -> CreateReply {
    let embed = create_xkcd_embed(&entry);
    let action_row = CreateActionRow::Buttons(vec![create_xkcd_button(&entry)]);

    CreateReply::default()
        .embed(embed)
        .components(vec![action_row])
}

/// XKCD commands.
#[poise::command(slash_command, subcommands("get", "random"), subcommand_required)]
pub async fn xkcd(_: NamelessContext<'_>) -> Result<(), NamelessError> {
    Ok(())
}

/// Get an XKCD comic.
#[poise::command(slash_command)]
pub async fn get(
    ctx: NamelessContext<'_>,
    #[description = "XKCD comic number, blank for latest."]
    #[min = 1]
    number: Option<u32>,
) -> Result<(), NamelessError> {
    // https://github.com/seanmonstar/reqwest/issues/1017
    let xkcd = match number {
        Some(num) => xkcd_nameless::fetch::fetch_number(num).await,
        None => xkcd_nameless::fetch::fetch_latest().await,
    }
    .expect("Unable to call XKCD API");

    ctx.send(create_xkcd_reply(xkcd)).await.unwrap();

    Ok(())
}

/// Get a random XKCD comic.
#[poise::command(slash_command)]
pub async fn random(ctx: NamelessContext<'_>) -> Result<(), NamelessError> {
    // https://github.com/seanmonstar/reqwest/issues/1017
    let xkcd = xkcd_nameless::fetch::fetch_random()
        .await
        .expect("Unable to call XKCD API");

    ctx.send(create_xkcd_reply(xkcd)).await.unwrap();

    Ok(())
}
