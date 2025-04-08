use async_trait::async_trait;
use serenity::all::{
    Colour, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, EditInteractionResponse, ResolvedOption, ResolvedValue,
};
use sqlx::{PgPool, Postgres};
use zayden_core::{SlashCommand, parse_options};

use crate::{Error, Result};

use super::{COIN, GamblingRow, GamblingTable, verify_bet, verify_cooldown};

pub struct Roll;

#[async_trait]
impl SlashCommand<Error, Postgres> for Roll {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let mut options = parse_options(options);

        let Some(ResolvedValue::String(dice)) = options.remove("dice") else {
            unreachable!("dice option is required")
        };

        let n_sides = dice.parse::<i64>().unwrap();

        let Some(ResolvedValue::Integer(prediction)) = options.remove("prediction") else {
            unreachable!("prediction option is required")
        };

        verify_prediction(prediction, 0, n_sides + 1)?;

        let mut row = GamblingRow::from_table(pool, interaction.user.id)
            .await
            .unwrap();

        verify_cooldown(&row)?;

        let Some(ResolvedValue::Integer(bet)) = options.remove("bet") else {
            unreachable!("bet option is required")
        };

        verify_bet(&row, bet)?;

        let roll = rand::random_range(1..=n_sides);

        let (title, result, payout, colour) = if roll == prediction {
            (
                "🎲 Dice Roll 🎲 - You Won!",
                "Profit:",
                bet * n_sides,
                Colour::DARK_GREEN,
            )
        } else {
            ("🎲 Dice Roll 🎲 - You Lost!", "Lost:", -bet, Colour::RED)
        };

        let cash = row.add_cash(payout);
        row.update_game();
        GamblingTable::save(pool, row).await.unwrap();

        let desc = format!(
            "Your bet: {bet} <:coin:{COIN}>\n\n**You picked:** {prediction} 🎲\n**Result:** {roll} 🎲\n\n{result} {payout}\nYour cash: {cash}",
        );

        let embed = CreateEmbed::new()
            .title(title)
            .description(desc)
            .colour(colour);

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed))
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        let cmd = CreateCommand::new("roll")
            .description("Roll the dice")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "dice",
                    "The type of dice to roll",
                )
                .add_string_choice("4-sides", "4")
                .add_string_choice("6-sides", "6")
                .add_string_choice("8-sides", "8")
                .add_string_choice("10-sides", "10")
                .add_string_choice("12-sides", "12")
                .add_string_choice("20-sides", "20")
                .required(true),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "prediction",
                    "What number will the dice land on?",
                )
                .required(true),
            )
            .add_option(
                CreateCommandOption::new(CommandOptionType::Integer, "bet", "Roll the dice")
                    .required(true),
            );

        Ok(cmd)
    }
}

fn verify_prediction(prediction: i64, min: i64, max: i64) -> Result<()> {
    if prediction > max || prediction < min {
        return Err(Error::InvalidPrediction);
    }

    Ok(())
}
