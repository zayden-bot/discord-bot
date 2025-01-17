use std::time::Duration;

use async_trait::async_trait;
use patreon_api::types::Member;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateEmbedFooter, Ready, ResolvedOption, ResolvedValue,
};
use sqlx::PgPool;
use url::Url;
use zayden_core::{parse_options, SlashCommand};

use crate::sqlx_lib::PostgresPool;
use crate::utils::{embed_response, message_response};
use crate::{Error, Result, SERVER_URL};

pub mod cache;
mod patreon_user;
pub use patreon_user::patreon_member;

const CLIENT_ID: &str = "co3TJ3lwqHN5WSVuIBiDNhfQv28V4FR-z6g-_fIogDzj_Um09DoWLGE5rvAJeTQd";

pub fn register(ctx: &Context, ready: &Ready) -> Result<Vec<CreateCommand>> {
    let commands = vec![Patreon::register(ctx, ready)?];

    Ok(commands)
}

pub struct Patreon;

#[async_trait]
impl SlashCommand<Error> for Patreon {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
    ) -> Result<()> {
        let pool = PostgresPool::get(ctx).await;

        let command = &options[0];

        match command.name {
            "info" => info(ctx, interaction).await?,
            "login" => login(ctx, interaction, &pool).await?,
            "check" => check(ctx, interaction, &pool, &command.value).await?,
            _ => unreachable!("Unknown subcommand"),
        };

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("patreon")
            .description("Patreon information")
            .add_option(CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "info",
                "Patreon information",
            ))
            .add_option(CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "login",
                "Login to Patreon",
            ))
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "check",
                    "Check if you're a patron",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "email",
                        "Your Patreon email",
                    )
                    .required(true),
                )
                .add_sub_option(CreateCommandOption::new(
                    CommandOptionType::Boolean,
                    "force",
                    "Check via the Patreon API instead of the cache",
                )),
            );

        Ok(command)
    }
}
async fn info(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer(ctx).await.unwrap();

    embed_response(ctx, interaction, CreateEmbed::new().title("Pledge to College Kings")
            .url("https://www.patreon.com/collegekings")
            .description("**Interested In Getting Early Updates, Patron-only behind the scenes/post... and more?\n\nCheck it all out here!**\nhttps://www.patreon.com/collegekings")
            .image("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg")
            .thumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png")
            .footer(CreateEmbedFooter::new("https://www.patreon.com/collegekings"))
    ).await.unwrap();

    Ok(())
}

async fn login(ctx: &Context, interaction: &CommandInteraction, pool: &PgPool) -> Result<()> {
    interaction.defer_ephemeral(ctx).await.unwrap();

    let patreon_url = Url::parse(&format!("https://www.patreon.com/oauth2/authorize?response_type=code&client_id={}&redirect_uri={}/api/v1/patreon/oauth/zayden&state={}", CLIENT_ID, SERVER_URL, interaction.user.id)).unwrap();

    message_response(ctx, interaction, patreon_url.as_str())
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_secs(5)).await;

    for _ in 0..10 * 60 {
        let Member { email, .. } = patreon_member(pool, &interaction.user.id.to_string(), false)
            .await?
            .unwrap()
            .data
            .attributes;

        // TODO: Fix this
        if email.is_some() {
            message_response(ctx, interaction, "Status: Success!")
                .await
                .unwrap();

            return Ok(());
        } else {
            message_response(
                ctx,
                interaction,
                format!("{patreon_url}\n\nStatus: User not currently in cache."),
            )
            .await
            .unwrap();
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    message_response(ctx, interaction, "Status: Timeout")
        .await
        .unwrap();

    Ok(())
}

async fn check(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &PgPool,
    subcommand: &ResolvedValue<'_>,
) -> Result<()> {
    interaction.defer_ephemeral(ctx).await.unwrap();

    let subcommand = match subcommand {
        ResolvedValue::SubCommand(subcommand) => subcommand,
        _ => unreachable!("Subcommand is required"),
    };

    let options = parse_options(subcommand);

    let email = match options.get("email") {
        Some(ResolvedValue::String(email)) => *email,
        _ => unreachable!("Email option is required"),
    };

    let force = match options.get("force") {
        Some(ResolvedValue::Boolean(force)) => *force,
        _ => false,
    };

    let Member {
        email,
        campaign_lifetime_support_cents,
        currently_entitled_amount_cents,
        ..
    } = patreon_member(pool, email, force)
        .await?
        .unwrap()
        .data
        .attributes;

    embed_response(
        ctx,
        interaction,
        CreateEmbed::new()
            .title("Patreon Status")
            .description(format!(
                "Email: {}\n Lifetime Support: **${}**\nCurrent Tier: **${}**",
                email.unwrap_or_default(),
                campaign_lifetime_support_cents / 100,
                currently_entitled_amount_cents / 100
            )),
    )
    .await
    .unwrap();

    Ok(())
}
