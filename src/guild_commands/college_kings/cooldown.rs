use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, Permissions,
};

use crate::{state::State, utils::message_response, Error, Result};

fn gifs_cooldown(content: &str) -> bool {
    content.starts_with("https://tenor.com")
}

async fn gifs(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let mut data = ctx.data.write().await;

    let state = data.get_mut::<State>().ok_or_else(|| Error::DataNotFound)?;

    if state.cooldown_conditions.remove("gifs").is_some() {
        message_response(ctx, interaction, "Gifs are no longer on cooldown").await?;
    } else {
        state
            .cooldown_conditions
            .insert(String::from("gifs"), gifs_cooldown);
        message_response(ctx, interaction, "Gifs are on cooldown").await?;
    }

    Ok(())
}

fn sydney_sweeney_gifs_cooldown(content: &str) -> bool {
    content.starts_with("https://tenor.com")
        && content.contains("sydney")
        && content.contains("sweeney")
}

async fn sydney_sweeney_gifs(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    let mut data = ctx.data.write().await;

    let state = data.get_mut::<State>().ok_or_else(|| Error::DataNotFound)?;

    if state
        .cooldown_conditions
        .remove("Sydney Sweeney gifs")
        .is_some()
    {
        message_response(
            ctx,
            interaction,
            "Sydney Sweeney gifs are no longer on cooldown",
        )
        .await?;
    } else {
        state.cooldown_conditions.insert(
            String::from("Sydney Sweeney gifs"),
            sydney_sweeney_gifs_cooldown,
        );
        message_response(ctx, interaction, "Sydney Sweeney gifs are on cooldown").await?;
    }

    Ok(())
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer_ephemeral(ctx).await?;

    let options = interaction.data.options();
    let command = &options[0];

    match command.name {
        "gifs" => gifs(ctx, interaction).await?,
        "sydney_sweeney" => sydney_sweeney_gifs(ctx, interaction).await?,
        _ => unreachable!("Invalid subcommand"),
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("cooldown")
        .description("Toggle cooldowns")
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "gifs",
            "Toggle gifs cooldown",
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "sydney_sweeney",
            "Toggle Sydney Sweeney gifs cooldown",
        ))
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
