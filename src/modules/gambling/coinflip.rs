use std::{fmt::Display, str::FromStr};

use async_trait::async_trait;
use serenity::all::{
    Colour, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, EditInteractionResponse, EmojiId, ResolvedOption, ResolvedValue,
};
use sqlx::{PgPool, Postgres};
use zayden_core::{SlashCommand, parse_options};

use crate::{Error, Result, sqlx_lib::GuildTable};

use super::{COIN, GamblingRow, GamblingTable, verify_bet, verify_cooldown};

const TAILS: EmojiId = EmojiId::new(1356741709995704600);

pub struct Coinflip;

#[async_trait]
impl SlashCommand<Error, Postgres> for Coinflip {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let mut options = parse_options(options);

        let Some(ResolvedValue::String(prediction)) = options.remove("prediction") else {
            unreachable!("prediction is required")
        };
        let prediction = prediction.parse::<CoinSide>().unwrap();

        let Some(ResolvedValue::Integer(mut bet)) = options.remove("bet") else {
            unreachable!("bet is required")
        };

        let mut row = GamblingTable::get(pool, interaction.user.id)
            .await
            .unwrap()
            .unwrap_or_else(|| GamblingRow::new(interaction.user.id));

        verify_cooldown(&row)?;
        verify_bet(&row, bet)?;

        let edge = rand::random_bool(1.0 / 6000.0);
        let mut winner = rand::random_bool(0.5);

        if edge {
            winner = true;
            bet *= 10000;
        }

        if winner {
            row.cash += bet;
        } else {
            row.cash -= bet;
        };

        row.update_game();

        let cash = row.cash;

        GamblingTable::save(pool, row).await.unwrap();

        let guild_id = interaction.guild_id.unwrap();

        if winner {
            GuildTable::update_stats(pool, guild_id, bet).await.unwrap();
        } else {
            GuildTable::update_stats(pool, guild_id, -bet)
                .await
                .unwrap();
        }

        let coin: CoinSide = if edge {
            CoinSide::Edge
        } else if winner {
            prediction
        } else {
            prediction.opposite()
        };

        let title = if edge {
            "Coin Flip - EDGE ROLL!"
        } else if winner {
            "Coin Flip - You Won!"
        } else {
            "Coin Flip - You Lost!"
        };

        let result = if winner {
            format!("Profit: {bet}")
        } else {
            format!("Lost: {bet}")
        };

        let desc = format!(
            "Your bet: {bet} <:coin:{COIN}>\n\n**You bet on:** {} ({prediction})\n**Coin landed on:** {} ({coin})\n\n{result}\nYour cash: {cash}",
            prediction.as_emoji(),
            coin.as_emoji(),
        );

        let colour = if winner {
            Colour::DARK_GREEN
        } else {
            Colour::RED
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
        let cmd = CreateCommand::new("coinflip")
            .description("Flip a coin!")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "prediction",
                    "Choose whether you think the coin will be heads or tails",
                )
                .add_string_choice("Heads", "heads")
                .add_string_choice("Tails", "tails")
                .required(true),
            )
            .add_option(
                CreateCommandOption::new(CommandOptionType::Integer, "bet", "The amount to bet.")
                    .required(true),
            );

        Ok(cmd)
    }
}

#[derive(Debug, Clone, Copy)]
enum CoinSide {
    Heads,
    Tails,
    Edge,
}

impl CoinSide {
    fn opposite(&self) -> CoinSide {
        match self {
            CoinSide::Heads => CoinSide::Tails,
            CoinSide::Tails => CoinSide::Heads,
            CoinSide::Edge => CoinSide::Edge,
        }
    }

    fn as_emoji(&self) -> String {
        match self {
            CoinSide::Heads => format!("<:heads:{COIN}>"),
            CoinSide::Tails => format!("<:tails:{TAILS}>"),
            CoinSide::Edge => format!("<:edge:{COIN}>"),
        }
    }
}

impl Display for CoinSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoinSide::Heads => write!(f, "Heads"),
            CoinSide::Tails => write!(f, "Tails"),
            CoinSide::Edge => write!(f, "Edge"),
        }
    }
}

impl FromStr for CoinSide {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "heads" => Ok(CoinSide::Heads),
            "tails" => Ok(CoinSide::Tails),
            _ => Err(()),
        }
    }
}
