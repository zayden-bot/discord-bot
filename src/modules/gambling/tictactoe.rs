use std::time::Duration;

use async_trait::async_trait;
use futures::StreamExt;
use serenity::all::{
    ButtonStyle, CommandInteraction, CommandOptionType, Context, CreateButton, CreateCommand,
    CreateCommandOption, CreateEmbed, EditInteractionResponse, Mentionable, ResolvedOption,
    ResolvedValue,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

use super::{COIN, GamblingRow, GamblingTable, verify_bet, verify_cooldown};

pub struct TicTacToe;

#[async_trait]
impl SlashCommand<Error, Postgres> for TicTacToe {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        mut options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let mut row = GamblingRow::from_table(pool, interaction.user.id)
            .await
            .unwrap();

        verify_cooldown(&row)?;

        let ResolvedValue::Integer(bet) = options.pop().unwrap().value else {
            unreachable!("bet is required option")
        };

        verify_bet(&row, bet)?;

        let embed = CreateEmbed::new().title("TicTacToe").description(format!(
            "{} wants to play tic-tac-toe for **{}** <:coin:{COIN}>",
            interaction.user.mention(),
            bet
        ));

        row.update_game();
        GamblingTable::save(pool, row).await.unwrap();

        let msg = interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .embed(embed)
                    .button(
                        CreateButton::new("ttt_accept")
                            .label("Accept")
                            .emoji('✅')
                            .style(ButtonStyle::Secondary),
                    )
                    .button(
                        CreateButton::new("ttt_cancel")
                            .label("Cancel")
                            .emoji('❌')
                            .style(ButtonStyle::Secondary),
                    ),
            )
            .await
            .unwrap();

        let mut stream = msg
            .await_component_interactions(ctx)
            .author_id(interaction.user.id)
            .timeout(Duration::from_secs(120))
            .stream();

        while let Some(interaction) = stream.next().await {}

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        Ok(CreateCommand::new("tictactoe")
            .description("Play a game of tic tac toe")
            .add_option(
                CreateCommandOption::new(CommandOptionType::Integer, "bet", "The amount to bet.")
                    .required(true),
            ))
    }
}
