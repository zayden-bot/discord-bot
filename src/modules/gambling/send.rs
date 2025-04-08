use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, EditInteractionResponse, Mentionable, ResolvedOption, ResolvedValue,
};
use sqlx::{PgPool, Postgres};
use zayden_core::{SlashCommand, parse_options};

use crate::{Error, Result};

use super::{COIN, GamblingRow, GamblingTable};

pub struct Send;

#[async_trait]
impl SlashCommand<Error, Postgres> for Send {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let mut options = parse_options(options);

        let Some(ResolvedValue::User(recipient, _)) = options.remove("recipient") else {
            unreachable!("recipient is required");
        };

        if recipient == &interaction.user {
            return Err(Error::SelfSend);
        }

        let Some(ResolvedValue::Integer(amount)) = options.remove("amount") else {
            unreachable!("amount is required");
        };

        if amount < 0 {
            return Err(Error::NegativeAmount);
        }

        let mut user_row = GamblingTable::get(pool, interaction.user.id)
            .await
            .unwrap()
            .unwrap_or_else(|| GamblingRow::new(interaction.user.id));

        if user_row.cash < amount {
            return Err(Error::InsufficientFunds);
        }

        user_row.cash -= amount;

        GamblingTable::save(pool, user_row).await.unwrap();

        if GamblingTable::add_cash(pool, recipient.id, amount)
            .await
            .is_err()
        {
            let mut row = GamblingRow::new(recipient.id);
            row.cash += amount;
            GamblingTable::save(pool, row).await.unwrap();
        }

        let embed = CreateEmbed::new().description(format!(
            "You sent {amount} <:coin:{COIN}> to {}",
            recipient.mention()
        ));

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed))
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        let cmd = CreateCommand::new("send")
            .description("Send another player some of your cash")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::User,
                    "recipient",
                    "The player recieving the cash",
                )
                .required(true),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "amount",
                    "The amount to send",
                )
                .required(true),
            );

        Ok(cmd)
    }
}
