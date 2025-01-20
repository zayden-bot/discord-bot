use std::time::Duration;

use async_trait::async_trait;
use futures::{StreamExt, TryStreamExt};
use serenity::all::{
    ButtonStyle, CommandInteraction, CommandOptionType, Context, CreateButton, CreateCommand,
    CreateCommandOption, CreateEmbed, CreateInteractionResponse, EditInteractionResponse, GuildId,
    Member, PermissionOverwrite, PermissionOverwriteType, Permissions, Ready, ResolvedOption,
    ResolvedValue,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::guilds::ServersTable;
use crate::handler::OnReady;
use crate::sqlx_lib::PostgresPool;
use crate::{Error, Result};

pub struct Sleep;

#[async_trait]
impl SlashCommand<Error, Postgres> for Sleep {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        mut options: Vec<ResolvedOption<'_>>,
        _pool: &PgPool,
    ) -> Result<()> {
        interaction.defer_ephemeral(ctx).await.unwrap();

        let Some(ResolvedValue::Integer(hours)) = options.pop().map(|option| option.value) else {
            unreachable!("Hours option is required");
        };

        let hours = hours.try_into().map_err(|_| Error::NegativeHours)?;

        let embed = CreateEmbed::new().description("This command will remove your ability to view any channels in the server until the time limit has expired or the zayden-bot restarts. This cannot be undone at this time.\nAre you sure you want to continue?");

        let confirm_button = CreateButton::new("sleep_confirm")
            .label("Confirm")
            .style(ButtonStyle::Success);
        let cancel_button = CreateButton::new("sleep_cancel")
            .label("Cancel")
            .style(ButtonStyle::Danger);

        let message = interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .embed(embed)
                    .button(confirm_button)
                    .button(cancel_button),
            )
            .await
            .unwrap();

        if let Some(interaction) = message
            .await_component_interaction(ctx)
            .timeout(Duration::from_secs(60))
            .await
        {
            interaction
                .create_response(ctx, CreateInteractionResponse::Acknowledge)
                .await
                .unwrap();

            if interaction.data.custom_id == "sleep_confirm" {
                let member = interaction.member.unwrap();

                let ctx_clone = ctx.clone();
                tokio::spawn(async move { sleep_user(ctx_clone, member, hours).await });
            }
        };

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("sleep")
            .description("Remove the distraction of discord for a while")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "hours",
                    "The number of hours to sleep for",
                )
                .required(true),
            );

        Ok(command)
    }
}

#[async_trait]
impl OnReady for Sleep {
    async fn on_ready(ctx: Context, ready: Ready) -> Result<()> {
        let pool = PostgresPool::get(&ctx).await;

        for guild in ready.guilds {
            reset_sleep(&ctx, &pool, guild.id).await?;
        }

        Ok(())
    }
}

async fn reset_sleep(ctx: &Context, pool: &PgPool, guild_id: GuildId) -> Result<()> {
    let sleep_role_id = match ServersTable::get_row(pool, guild_id).await.unwrap() {
        Some(row) => match row.sleep_role_id() {
            Some(id) => id,
            None => return Ok(()),
        },
        None => return Ok(()),
    };

    let channels = guild_id.channels(&ctx).await.unwrap();
    for channel in channels.values() {
        if channel.name == "sleeping" {
            continue;
        }

        channel
            .create_permission(
                &ctx,
                PermissionOverwrite {
                    allow: Permissions::empty(),
                    deny: Permissions::VIEW_CHANNEL,
                    kind: PermissionOverwriteType::Role(sleep_role_id),
                },
            )
            .await
            .unwrap();
    }

    let mut members = guild_id.members_iter(&ctx).boxed();

    while let Some(member) = members.try_next().await.unwrap() {
        if member.roles.contains(&sleep_role_id) {
            member.remove_role(&ctx, sleep_role_id).await.unwrap();
        }
    }

    Ok(())
}

async fn sleep_user(ctx: Context, member: Member, hours: u64) -> Result<()> {
    let pool = PostgresPool::get(&ctx).await;

    let sleep_role_id = ServersTable::get_row(&pool, member.guild_id)
        .await
        .unwrap()
        .unwrap()
        .sleep_role_id()
        .unwrap();

    drop(pool);

    member.add_role(&ctx, sleep_role_id).await.unwrap();

    tokio::time::sleep(Duration::from_secs(hours * 60 * 60)).await;

    println!("Waking up {}", member.user.name);

    member.remove_role(&ctx, sleep_role_id).await.unwrap();

    Ok(())
}
