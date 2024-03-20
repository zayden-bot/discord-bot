use crate::{
    utils::{embed_response, message_response, parse_options},
    SERVER_URL,
};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateEmbedFooter, Message, ResolvedValue,
};

#[derive(Deserialize, Debug)]
pub struct MemberAttributes {
    pub currently_entitled_amount_cents: Option<i32>,
    pub email: Option<String>,
    pub lifetime_support_cents: Option<i32>,
}

async fn info(ctx: &Context, interaction: &CommandInteraction) -> Result<Message, serenity::Error> {
    interaction.defer(ctx).await?;

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
    subcommand: &ResolvedValue<'_>,
) -> Result<Message, serenity::Error> {
    interaction.defer_ephemeral(ctx).await?;

    let subcommand = match subcommand {
        ResolvedValue::SubCommand(subcommand) => subcommand,
        _ => return message_response(ctx, interaction, "Invalid subcommand").await,
    };

    let options = parse_options(subcommand);

    let email = match options.get("email") {
        Some(ResolvedValue::String(email)) => email,
        _ => return message_response(ctx, interaction, "Invalid email").await,
    };

    let force = match options.get("force") {
        Some(ResolvedValue::Boolean(force)) => *force,
        _ => false,
    };

    let res = match Client::new()
        .post(&format!("{}/api/v1/patreon/get_user", SERVER_URL))
        .json(&json!({ "email": email, "force": force}))
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => {
            return message_response(ctx, interaction, "Failed to fetch patreon data").await;
        }
    };

    let res_text = res.text().await?;

    let attributes: MemberAttributes = match serde_json::from_str(&res_text) {
        Ok(attributes) => attributes,
        Err(_) => {
            eprintln!("Failed to parse patreon data: {}", res_text);
            return message_response(ctx, interaction, "Failed to parse patreon data").await;
        }
    };

    embed_response(
        ctx,
        interaction,
        CreateEmbed::new()
            .title("Patreon Status")
            .description(format!(
                "Email: {}\n Lifetime Support: **${}**\nCurrent Tier: **${}**",
                attributes.email.unwrap(),
                attributes.lifetime_support_cents.unwrap() / 100,
                attributes.currently_entitled_amount_cents.unwrap() / 100
            )),
    )
    .await
}

pub async fn run(
    ctx: Context,
    interaction: &CommandInteraction,
) -> Result<Message, serenity::Error> {
    let options = interaction.data.options();
    let command = &options[0];

    match command.name {
        "info" => info(&ctx, interaction).await,
        "check" => check(&ctx, interaction, &command.value).await,
        _ => message_response(&ctx, interaction, "Invalid subcommand").await,
    }
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
            )
            .add_sub_option(CreateCommandOption::new(
                CommandOptionType::Boolean,
                "force",
                "Check via the Patreon API instead of the cache",
            )),
        )
}
