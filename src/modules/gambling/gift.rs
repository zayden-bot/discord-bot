use async_trait::async_trait;
use chrono::Utc;
use serenity::all::{
    Colour, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, EditInteractionResponse, Mentionable, ResolvedOption, ResolvedValue,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

use super::{GamblingRow, GamblingTable};

pub struct Gift;

#[async_trait]
impl SlashCommand<Error, Postgres> for Gift {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let ResolvedValue::User(recipient, _) = options[0].value else {
            unreachable!("recipient is required")
        };

        if recipient == &interaction.user {
            return Err(Error::SelfGift);
        }

        let mut user_row = GamblingTable::get(pool, interaction.user.id)
            .await
            .unwrap()
            .unwrap_or_else(|| GamblingRow::new(interaction.user.id));

        let today = Utc::now().naive_utc().date();

        if user_row.gift == today {
            return Err(Error::GiftUsed);
        }

        user_row.gift = today;
        GamblingTable::save(pool, user_row).await.unwrap();

        if (GamblingTable::add_cash(pool, recipient.id, 2500).await).is_err() {
            let mut row = GamblingRow::new(recipient.id);
            row.cash += 2500;
            GamblingTable::save(pool, row).await.unwrap();
        };

        let embed = CreateEmbed::new()
            .description(format!(
                "🎁 You sent a gift of 2,500 to {}",
                recipient.mention()
            ))
            .colour(Colour::GOLD);

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed))
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        let cmd = CreateCommand::new("gift")
            .description("Send a free gift to a user!")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::User,
                    "recipient",
                    "The user to receive the free gift",
                )
                .required(true),
            );

        Ok(cmd)
    }
}
