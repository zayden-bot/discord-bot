#![allow(dead_code)]

use crate::utils::{embed_response, message_response};
use serde::Deserialize;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, CreateEmbed, CreateEmbedFooter, Message,
};

#[derive(Debug, Deserialize)]
struct PatreonMemberAttributes {
    campaign_lifetime_support_cents: i32,
    email: Option<String>,
    patron_status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PatreonMemberData {
    attributes: PatreonMemberAttributes,
    id: Option<String>,
    r#type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PatreonLinks {
    next: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PatreonPagination {
    total: i32,
}

#[derive(Debug, Deserialize)]
struct PatreonMeta {
    pagination: Option<PatreonPagination>,
}

#[derive(Debug, Deserialize)]
struct PatreonMember {
    data: Vec<PatreonMemberData>,
    patreon_links: Option<PatreonLinks>,
    patreon_meta: Option<PatreonMeta>,
}

async fn info(ctx: &Context, interaction: &CommandInteraction) -> Result<Message, serenity::Error> {
    embed_response(ctx, interaction, CreateEmbed::new().title("Pledge to College Kings")
            .url("https://www.patreon.com/collegekings")
            .description("**Interested In Getting Early Updates, Patron-only behind the scenes/post... and more?\n\nCheck it all out here!**\nhttps://www.patreon.com/collegekings")
            .image("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg")
            .thumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png")
            .footer(CreateEmbedFooter::new("https://www.patreon.com/collegekings"))
    ).await
}

async fn check(
    ctx: &Context,
    interaction: &CommandInteraction,
    subcommand: &CommandDataOptionValue,
) -> Result<Message, serenity::Error> {
    let subcommand = match subcommand {
        CommandDataOptionValue::SubCommand(subcommand) => subcommand,
        _ => return message_response(ctx, interaction, "Invalid subcommand").await,
    };

    let email = match subcommand[0].value.as_str() {
        Some(email) => email,
        _ => return message_response(ctx, interaction, "Invalid email").await,
    };

    let res = match reqwest::get(format!(
        "http://81.100.246.35/api/v1/patreon/users/{}",
        email
    ))
    .await
    {
        Ok(res) => res,
        Err(_) => {
            return message_response(ctx, interaction, "Error getting Patreon information").await
        }
    };

    let patreon_member = res.json::<PatreonMember>().await.unwrap();
    let patreon_attributes = &patreon_member.data[0].attributes;

    embed_response(
        ctx,
        interaction,
        CreateEmbed::new()
            .title("Patreon Status")
            .description(format!(
                "Lifetime Support (USD): **{}**\nEmail: {}\nPatreon Status: **{}**",
                patreon_attributes.campaign_lifetime_support_cents / 100,
                patreon_attributes.email.as_ref().unwrap(),
                patreon_attributes.patron_status.as_ref().unwrap()
            )),
    )
    .await
}

pub async fn run(
    ctx: Context,
    interaction: &CommandInteraction,
) -> Result<Message, serenity::Error> {
    let command = &interaction.data.options[0];
    println!("{:?}", interaction.data.options);

    return match command.name.as_str() {
        "info" => info(&ctx, interaction).await,
        "check" => check(&ctx, interaction, &command.value).await,
        _ => message_response(&ctx, interaction, "Invalid subcommand").await,
    };
}

pub fn register() -> CreateCommand {
    CreateCommand::new("patreon")
        .description("Patreon information")
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "info",
            "Patreon information",
        ))
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "check",
                "Check if you're a patron",
            )
            .add_sub_option(
                CreateCommandOption::new(CommandOptionType::String, "email", "Your Patreon email")
                    .required(true),
            ),
        )
}
