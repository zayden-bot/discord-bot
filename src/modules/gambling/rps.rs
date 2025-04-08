use std::{fmt::Display, str::FromStr};

use async_trait::async_trait;
use rand::seq::IndexedRandom;
use serenity::all::{
    Colour, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, EditInteractionResponse, ResolvedOption, ResolvedValue,
};
use sqlx::{PgPool, Postgres};
use zayden_core::{SlashCommand, parse_options};

use crate::{Error, Result, sqlx_lib::GuildTable};

use super::{COIN, GamblingRow, GamblingTable, verify_bet, verify_cooldown};

pub struct RPS;

#[async_trait]
impl SlashCommand<Error, Postgres> for RPS {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let mut options = parse_options(options);

        let Some(ResolvedValue::String(selection)) = options.remove("selection") else {
            unreachable!("selection is required")
        };
        let user_choice = selection.parse::<RPSChoice>().unwrap();

        let Some(ResolvedValue::Integer(bet)) = options.remove("bet") else {
            unreachable!("bet is required")
        };

        let mut row = GamblingTable::get(pool, interaction.user.id)
            .await
            .unwrap()
            .unwrap_or_else(|| GamblingRow::new(interaction.user.id));

        verify_cooldown(&row)?;
        verify_bet(&row, bet)?;

        let computer_choice = *CHOICES.choose(&mut rand::rng()).unwrap();
        let winner = user_choice.winner(&computer_choice);

        if winner == Some(true) {
            row.cash += bet;
        } else if winner == Some(false) {
            row.cash -= bet;
        }

        row.update_game();

        let cash = GamblingTable::save(pool, row).await.unwrap().cash;

        let guild_id = interaction.guild_id.unwrap();

        if winner == Some(true) {
            GuildTable::update_stats(pool, guild_id, bet).await.unwrap();
        } else if winner == Some(false) {
            GuildTable::update_stats(pool, guild_id, -bet)
                .await
                .unwrap();
        }

        let title = if winner == Some(true) {
            "Rock 🪨 Paper 🗞️ Scissors ✂ - You Won!"
        } else if winner == Some(false) {
            "Rock 🪨 Paper 🗞️ Scissors ✂ - You Lost!"
        } else {
            "Rock 🪨 Paper 🗞️ Scissors ✂ - You Tied!"
        };

        let result = if winner == Some(true) {
            format!("Profit: {bet}")
        } else if winner == Some(false) {
            format!("Lost: {bet}")
        } else {
            "No change".to_string()
        };

        let desc = format!(
            "Your bet: {bet} <:coin:{COIN}>\n\n**You picked:** {}\n**Zayden picked:** {}\n\n{result}\nYour cash: {cash}",
            user_choice.as_emoji(),
            computer_choice.as_emoji(),
        );

        let colour = if winner == Some(true) {
            Colour::DARK_GREEN
        } else if winner == Some(false) {
            Colour::RED
        } else {
            Colour::DARKER_GREY
        };

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
        let cmd = CreateCommand::new("rps")
            .description("Play a game of rock paper scissors against the bot")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "selection",
                    "Your choice of Rock, Paper or Scissors",
                )
                .required(true)
                .add_string_choice("Rock", "rock")
                .add_string_choice("Paper", "paper")
                .add_string_choice("Scissors", "scissors"),
            )
            .add_option(
                CreateCommandOption::new(CommandOptionType::Integer, "bet", "The amount to bet.")
                    .required(true),
            );

        Ok(cmd)
    }
}

const CHOICES: [RPSChoice; 3] = [RPSChoice::Rock, RPSChoice::Paper, RPSChoice::Scissors];

#[derive(Clone, Copy, PartialEq, Eq)]
enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

impl RPSChoice {
    fn winner(&self, opponent: &Self) -> Option<bool> {
        match (self, opponent) {
            (a, b) if a == b => None,
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => Some(true),
            _ => Some(false),
        }
    }

    fn as_emoji(&self) -> &str {
        match self {
            Self::Rock => "🪨",
            Self::Paper => "🗞️",
            Self::Scissors => "✂",
        }
    }
}

impl Display for RPSChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RPSChoice::Rock => write!(f, "Rock"),
            RPSChoice::Paper => write!(f, "Paper"),
            RPSChoice::Scissors => write!(f, "Scissors"),
        }
    }
}

impl FromStr for RPSChoice {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "rock" => Ok(Self::Rock),
            "paper" => Ok(Self::Paper),
            "scissors" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}
