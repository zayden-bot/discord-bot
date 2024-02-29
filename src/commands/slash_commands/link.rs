use crate::utils::{message_response, parse_options};
use serde::Deserialize;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, Message,
    Permissions, ResolvedValue,
};

#[derive(Deserialize, Debug)]
pub struct MemberAttributes {
    pub currently_entitled_amount_cents: Option<i32>,
    pub email: Option<String>,
    pub lifetime_support_cents: Option<i32>,
}

async fn download(
    ctx: &Context,
    interaction: &CommandInteraction,
    subcommand: &ResolvedValue<'_>,
) -> Result<Message, serenity::Error> {
    interaction.defer(ctx).await?;

    let subcommand = match subcommand {
        ResolvedValue::SubCommand(subcommand) => subcommand,
        _ => return message_response(ctx, interaction, "Invalid subcommand").await,
    };

    let options = parse_options(subcommand);

    let game = match options.get("game") {
        Some(ResolvedValue::String(game)) => game,
        _ => return message_response(ctx, interaction, "Invalid game").await,
    };

    let mut game_folder = game.to_lowercase();
    if game_folder == "college_kings" {
        game_folder = "college_kings_1".into();
    }

    let platform = match options.get("platform") {
        Some(ResolvedValue::String(platform)) => platform,
        _ => return message_response(ctx, interaction, "Invalid platform").await,
    };

    let version = match options.get("version") {
        Some(ResolvedValue::Integer(version)) => version
            .to_string()
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("."),
        _ => return message_response(ctx, interaction, "Invalid version").await,
    };

    let link = format!("https://collegekings.b-cdn.net/__bcdn_perma_cache__/pullzone__collegekings__22373407/wp-content/uploads/secured/{}/{}-{}-{}.zip", game_folder, game.replace('_', ""), version, platform);

    message_response(ctx, interaction, link).await
}

pub async fn run(
    ctx: Context,
    interaction: &CommandInteraction,
) -> Result<Message, serenity::Error> {
    let options = interaction.data.options();
    let command = &options[0];

    download(&ctx, interaction, &command.value).await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("link")
        .description("Helper command for getting links")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "download",
                "Direct download link",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "game",
                    "The game to get the download link for",
                )
                .add_string_choice("College Kings 1", "College_Kings")
                .add_string_choice("College Kings 2", "College_Kings_2")
                .required(true),
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "platform",
                    "The platform to get the download link for",
                )
                .add_string_choice("Windows", "pc")
                .add_string_choice("Mac", "mac")
                .add_string_choice("Linux", "pc")
                .required(true),
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "version",
                    "The version to get the download link for",
                )
                .required(true),
            ),
        )
        .default_member_permissions(Permissions::MANAGE_MESSAGES)
}
