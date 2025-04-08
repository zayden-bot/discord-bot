use std::time::Duration;

use async_trait::async_trait;
use futures::StreamExt;
use rand::seq::IndexedRandom;
use serenity::all::{
    Colour, CommandInteraction, ComponentInteraction, Context, CreateButton, CreateCommand,
    CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
    EditInteractionResponse, ResolvedOption,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result, sqlx_lib::GuildTable};

use super::{GamblingRow, GamblingTable, verify_cooldown};

const BUYIN: i64 = 100;

pub struct HigherLower;

#[async_trait]
impl SlashCommand<Error, Postgres> for HigherLower {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &PgPool,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let mut row = GamblingTable::get(pool, interaction.user.id)
            .await
            .unwrap()
            .unwrap_or_else(|| GamblingRow::new(interaction.user.id));

        if row.cash < BUYIN {
            return Err(Error::InsufficientFunds);
        }

        verify_cooldown(&row)?;
        row.cash -= BUYIN;

        let embed = create_embed(&rand::random_range(1..=15).to_string(), -BUYIN, true);

        let higher_btn = CreateButton::new("higher").emoji('☝').label("Higher");
        let lower_btn = CreateButton::new("lower").emoji('👇').label("Lower");

        let msg = interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .embed(embed)
                    .button(higher_btn)
                    .button(lower_btn),
            )
            .await
            .unwrap();

        let mut stream = msg
            .await_component_interactions(ctx)
            .author_id(interaction.user.id)
            .timeout(Duration::from_secs(120))
            .stream();

        let mut payout = -BUYIN;
        let mut prev_seq = String::new();

        while let Some(interaction) = stream.next().await {
            let mut desc_iter = interaction
                .message
                .embeds
                .first()
                .unwrap()
                .description
                .as_deref()
                .unwrap()
                .split("\n\n");

            prev_seq = desc_iter.next().unwrap().to_string();
            let prev_num = prev_seq.split(' ').last().unwrap().parse::<u8>().unwrap();

            let next_num = {
                let possible_nums = (1..=15).filter(|num| *num != prev_num).collect::<Vec<_>>();
                let mut rng = rand::rng();
                *possible_nums.choose(&mut rng).unwrap()
            };

            payout = desc_iter
                .next()
                .unwrap()
                .strip_prefix("Current Payout: ")
                .unwrap()
                .parse::<i64>()
                .unwrap();

            let choice = interaction.data.custom_id.as_str();

            let winner = if choice == "higher" {
                higher(ctx, &interaction, &mut prev_seq, prev_num, next_num, payout).await
            } else {
                lower(ctx, &interaction, &mut prev_seq, prev_num, next_num, payout).await
            };

            if !winner {
                break;
            }
        }

        GuildTable::update_stats(pool, interaction.guild_id.unwrap(), payout)
            .await
            .unwrap();

        row.cash += payout + BUYIN;
        let colour = if payout > 0 {
            Colour::DARK_GREEN
        } else {
            Colour::RED
        };

        row.update_game();

        let cash = row.cash;

        GamblingTable::save(pool, row).await.unwrap();

        let result = if payout > 0 {
            format!("Profit: {payout}")
        } else {
            format!("Lost: {payout}")
        };

        let embed = CreateEmbed::new()
            .title("Higher or Lower")
            .description(format!(
                "{}\n\nFinal Payout: {}\n\nThis game has ended.\n\n{result}\nYour cash: {cash}",
                prev_seq, payout
            ))
            .colour(colour);

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .embed(embed)
                    .components(Vec::new()),
            )
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context) -> Result<CreateCommand> {
        let cmd = CreateCommand::new("higherorlower").description("Play a game of higher or lower");

        Ok(cmd)
    }
}

fn create_embed(seq: &str, payout: i64, winner: bool) -> CreateEmbed {
    let desc = if winner {
        format!("{seq}\n\nCurrent Payout: {payout}\n\nGuess the next number!")
    } else {
        format!("{seq}\n\nFinal Payout: {payout}")
    };

    CreateEmbed::new()
        .title("Higher or Lower")
        .description(desc)
        .colour(Colour::TEAL)
}

async fn higher(
    ctx: &Context,
    interaction: &ComponentInteraction,
    seq: &mut String,
    prev: u8,
    next: u8,
    mut payout: i64,
) -> bool {
    seq.push(' ');

    let winner = next > prev;

    if winner {
        seq.push('☝');
        payout += 100
    } else {
        seq.push('❌');
    }

    seq.push_str(&format!(" {next}"));

    let embed = create_embed(seq, payout, winner);

    let msg = if winner {
        CreateInteractionResponseMessage::new().embed(embed)
    } else {
        CreateInteractionResponseMessage::new()
            .embed(embed)
            .components(Vec::new())
    };

    interaction
        .create_response(ctx, CreateInteractionResponse::UpdateMessage(msg))
        .await
        .unwrap();

    winner
}

async fn lower(
    ctx: &Context,
    interaction: &ComponentInteraction,
    seq: &mut String,
    prev: u8,
    next: u8,
    mut payout: i64,
) -> bool {
    seq.push(' ');

    let winner = next < prev;

    if winner {
        seq.push('👇');
        payout += 100
    } else {
        seq.push('❌');
    }

    seq.push_str(&format!(" {next}"));

    let embed = create_embed(seq, payout, winner);

    let msg = if winner {
        CreateInteractionResponseMessage::new().embed(embed)
    } else {
        CreateInteractionResponseMessage::new()
            .embed(embed)
            .components(Vec::new())
    };

    interaction
        .create_response(ctx, CreateInteractionResponse::UpdateMessage(msg))
        .await
        .unwrap();

    winner
}
