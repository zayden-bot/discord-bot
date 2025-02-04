use std::time::Duration;

use async_trait::async_trait;
use rand::rng;
use rand::seq::IndexedRandom;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateAttachment, CreateCommand,
    CreateCommandOption, CreateEmbed, EditAttachments, EditInteractionResponse, Ready,
    ResolvedOption, UserId,
};
use serenity::prelude::TypeMapKey;
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::guilds::ServersTable;
use crate::image_cache::ImageCache;
use crate::{Error, Result};

const ONE_HOUR: u64 = 60 * 60;

pub struct GreetingLockedUsers;

impl TypeMapKey for GreetingLockedUsers {
    type Value = Vec<UserId>;
}

pub struct Greetings;

#[async_trait]
impl SlashCommand<Error, Postgres> for Greetings {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        mut options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(&ctx).await.unwrap();

        let mut data = ctx.data.write().await;
        let locked_users = data.get_mut::<GreetingLockedUsers>().unwrap();

        let guild_id = interaction.guild_id.ok_or(Error::MissingGuildId)?;

        let general_channel_id = ServersTable::get_row(pool, guild_id)
            .await
            .unwrap()
            .unwrap()
            .get_general_channel_id()
            .unwrap();

        let user_id = interaction.user.id;

        if interaction.channel_id == general_channel_id {
            if locked_users.contains(&user_id) {
                return Err(Error::CommandTimeout);
            }

            locked_users.push(user_id);
        }

        let image_cache = data.get::<ImageCache>().unwrap();

        let subcommand = options.pop().unwrap();

        let entries = if subcommand.name == "morning" {
            &image_cache.good_morning_images
        } else {
            &image_cache.good_night_images
        };

        let image_path = entries.choose(&mut rng()).unwrap();
        let file_name = image_path.file_name().unwrap().to_str().unwrap();

        let title = if subcommand.name == "morning" {
            format!("Good Morning, {}!", interaction.user.name)
        } else {
            format!("Good Night, {}!", interaction.user.name)
        };

        interaction
            .edit_response(
                &ctx,
                EditInteractionResponse::new()
                    .embed(CreateEmbed::new().title(title).attachment(file_name))
                    .attachments(
                        EditAttachments::new()
                            .add(CreateAttachment::path(image_path).await.unwrap()),
                    ),
            )
            .await
            .unwrap();

        if interaction.channel_id == general_channel_id {
            tokio::spawn({
                let ctx = ctx.clone();
                async move {
                    tokio::time::sleep(Duration::from_secs(ONE_HOUR)).await;
                    let mut data = ctx.data.write().await;
                    if let Some(locked_users) = data.get_mut::<GreetingLockedUsers>() {
                        locked_users.retain(|x| *x != user_id);
                    }
                }
            });
        }

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let morning = CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "morning",
            "Have a CK girl bless your morning",
        );

        let night = CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "night",
            "Have a CK girl wish you a good night",
        );

        let command = CreateCommand::new("good")
            .description("Good morning or good night")
            .add_option(morning)
            .add_option(night);

        Ok(command)
    }
}
