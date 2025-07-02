use std::time::Duration;

use async_trait::async_trait;
use futures::StreamExt;
use serenity::all::{
    Colour, CommandInteraction, CommandOptionType, ComponentInteraction, Context, CreateButton,
    CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage, EditInteractionResponse, Mentionable, Message,
    ResolvedOption, ResolvedValue, UserId,
};
use sqlx::{Database, Pool, prelude::FromRow};
use zayden_core::{FormatNum, cache::GuildMembersCache};

use crate::shop::{EGGPLANT, LOTTO_TICKET};
use crate::{Coins, Gems, Result};

use super::Commands;

#[async_trait]
pub trait LeaderboardManager<Db: Database> {
    async fn networth(
        pool: &Pool<Db>,
        users: &[i64],
        page_num: i64,
    ) -> sqlx::Result<Vec<LeaderboardRow>>;

    async fn networth_row_number(
        pool: &Pool<Db>,
        id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Option<i64>>;

    async fn coins(
        pool: &Pool<Db>,
        users: &[i64],
        page_num: i64,
    ) -> sqlx::Result<Vec<LeaderboardRow>>;

    async fn coins_row_number(
        pool: &Pool<Db>,
        id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Option<i64>>;

    async fn gems(
        pool: &Pool<Db>,
        users: &[i64],
        page_num: i64,
    ) -> sqlx::Result<Vec<LeaderboardRow>>;

    async fn gems_row_number(
        pool: &Pool<Db>,
        id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Option<i64>>;

    async fn eggplants(
        pool: &Pool<Db>,
        users: &[i64],
        page_num: i64,
    ) -> sqlx::Result<Vec<LeaderboardRow>>;

    async fn eggplants_row_number(
        pool: &Pool<Db>,
        id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Option<i64>>;

    async fn lottotickets(pool: &Pool<Db>, page_num: i64) -> sqlx::Result<Vec<LeaderboardRow>>;

    async fn lottotickets_row_number(
        pool: &Pool<Db>,
        id: impl Into<UserId> + Send,
    ) -> sqlx::Result<Option<i64>>;
}

#[derive(FromRow)]
pub struct NetworthRow {
    pub id: i64,
    pub networth: Option<i64>,
}

#[derive(FromRow)]
pub struct CoinsRow {
    pub id: i64,
    pub coins: i64,
}

impl Coins for CoinsRow {
    fn coins(&self) -> i64 {
        self.coins
    }

    fn coins_mut(&mut self) -> &mut i64 {
        &mut self.coins
    }
}

#[derive(FromRow)]
pub struct GemsRow {
    pub id: i64,
    pub gems: i64,
}

impl Gems for GemsRow {
    fn gems(&self) -> i64 {
        self.gems
    }

    fn gems_mut(&mut self) -> &mut i64 {
        &mut self.gems
    }
}

#[derive(FromRow)]
pub struct EggplantsRow {
    pub user_id: i64,
    pub quantity: i64,
}

#[derive(FromRow)]
pub struct LottoTicketRow {
    pub user_id: i64,
    pub quantity: i64,
}

impl Commands {
    pub async fn leaderboard<Db: Database, Manager: LeaderboardManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        mut options: Vec<ResolvedOption<'_>>,
        pool: &Pool<Db>,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        let ResolvedValue::String(leaderboard) = options.pop().unwrap().value else {
            unreachable!("leaderboard option is required")
        };

        let users = {
            let data = ctx.data.read().await;
            let cache = data.get::<GuildMembersCache>().unwrap();
            cache
                .get(&interaction.guild_id.unwrap())
                .unwrap()
                .iter()
                .map(|id| id.get() as i64)
                .collect::<Vec<_>>()
        };

        let rows = get_rows::<Db, Manager>(leaderboard, pool, &users, 1).await;

        let desc = rows
            .into_iter()
            .enumerate()
            .map(|(i, row)| row.as_desc(i))
            .collect::<Vec<_>>()
            .join("\n\n");

        let embed = CreateEmbed::new()
            .title(format!("🏁 Leaderboard ({leaderboard})"))
            .description(desc)
            .footer(CreateEmbedFooter::new("Page 1"))
            .colour(Colour::TEAL);

        let mut response = EditInteractionResponse::new()
            .embed(embed)
            .button(CreateButton::new("leaderboard_previous").label("<"));

        if get_row_number::<Db, Manager>(leaderboard, pool, interaction.user.id)
            .await
            .is_some()
        {
            response = response.button(CreateButton::new("leaderboard_user").emoji('🎯'));
        }

        let msg = interaction
            .edit_response(
                ctx,
                response.button(CreateButton::new("leaderboard_next").label(">")),
            )
            .await
            .unwrap();

        let mut stream = msg
            .await_component_interactions(ctx)
            .timeout(Duration::from_secs(120))
            .stream();

        while let Some(component) = stream.next().await {
            run_component::<Db, Manager>(ctx, pool, &users, &msg, component).await?;
        }

        interaction
            .edit_response(ctx, EditInteractionResponse::new().components(Vec::new()))
            .await?;

        Ok(())
    }

    pub fn register_leaderboard() -> CreateCommand {
        CreateCommand::new("leaderboard")
            .description("The server leaderboard")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "leaderboard",
                    "The leaderboard to choose",
                )
                .required(true)
                .add_string_choice("Net Worth", "networth")
                .add_string_choice("Coins", "coins")
                .add_string_choice("Gems", "gems")
                .add_string_choice(EGGPLANT.name, "eggplants")
                .add_string_choice(LOTTO_TICKET.name, "lottotickets"),
            )
    }
}

async fn run_component<Db: Database, Manager: LeaderboardManager<Db>>(
    ctx: &Context,
    pool: &Pool<Db>,
    users: &[i64],
    msg: &Message,
    interaction: ComponentInteraction,
) -> Result<()> {
    let custom_id = interaction
        .data
        .custom_id
        .strip_prefix("leaderboard_")
        .unwrap();

    let embed = msg.embeds.first().unwrap();

    let leaderboard = embed
        .title
        .as_ref()
        .unwrap()
        .strip_prefix("🏁 Leaderboard (")
        .unwrap()
        .strip_suffix(")")
        .unwrap();

    let mut page_number: i64 = embed
        .footer
        .as_ref()
        .unwrap()
        .text
        .strip_prefix("Page ")
        .unwrap()
        .parse()
        .unwrap();

    match custom_id {
        "previous" => {
            page_number = (page_number - 1).max(1);
        }
        "user" => {
            let row_num = get_row_number::<Db, Manager>(leaderboard, pool, interaction.user.id)
                .await
                .unwrap();
            page_number = row_num / 10 + 1;
        }
        "next" => {
            page_number += 1;
        }
        _ => unreachable!("Invalid custom id"),
    };

    let rows = get_rows::<Db, Manager>(leaderboard, pool, users, page_number).await;

    let desc = rows
        .into_iter()
        .enumerate()
        .map(|(i, row)| row.as_desc(i + (page_number as usize - 1) * 10))
        .collect::<Vec<_>>()
        .join("\n\n");

    let embed = CreateEmbed::from(embed.clone())
        .footer(CreateEmbedFooter::new(format!("Page {}", page_number)))
        .description(desc);

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new().embed(embed),
            ),
        )
        .await
        .unwrap();

    Ok(())
}

pub enum LeaderboardRow {
    NetWorth(NetworthRow),
    Coins(CoinsRow),
    Gems(GemsRow),
    Eggplants(EggplantsRow),
    LottoTickets(LottoTicketRow),
}

impl LeaderboardRow {
    pub fn user_id(&self) -> UserId {
        match self {
            Self::NetWorth(row) => UserId::new(row.id as u64),
            Self::Coins(row) => UserId::new(row.id as u64),
            Self::Gems(row) => UserId::new(row.id as u64),
            Self::Eggplants(row) => UserId::new(row.user_id as u64),
            Self::LottoTickets(row) => UserId::new(row.user_id as u64),
        }
    }

    pub fn as_desc(&self, i: usize) -> String {
        let place = if i == 0 {
            "🥇".to_string()
        } else if i == 1 {
            "🥈".to_string()
        } else if i == 2 {
            "🥉".to_string()
        } else {
            format!("#{}", i + 1)
        };

        let data = match self {
            Self::NetWorth(row) => row.networth.unwrap_or_default().format(),
            Self::Coins(row) => row.coins_str(),
            Self::Gems(row) => row.gems_str(),
            Self::Eggplants(row) => format!("{} {}", row.quantity.format(), EGGPLANT.emoji()),
            Self::LottoTickets(row) => {
                format!("{} {}", row.quantity.format(), LOTTO_TICKET.emoji())
            }
        };

        format!("{place} - {} - {data}", self.user_id().mention())
    }
}

async fn get_rows<Db: Database, Manager: LeaderboardManager<Db>>(
    leaderboard: &str,
    pool: &Pool<Db>,
    users: &[i64],
    page_num: i64,
) -> Vec<LeaderboardRow> {
    match leaderboard {
        "networth" => Manager::networth(pool, users, page_num).await.unwrap(),
        "coins" => Manager::coins(pool, users, page_num).await.unwrap(),
        "gems" => Manager::gems(pool, users, page_num).await.unwrap(),
        "eggplants" => Manager::eggplants(pool, users, page_num).await.unwrap(),
        "lottotickets" => Manager::lottotickets(pool, page_num).await.unwrap(),
        _ => unreachable!("Invalid leaderboard option"),
    }
}

async fn get_row_number<Db: Database, Manager: LeaderboardManager<Db>>(
    leaderboard: &str,
    pool: &Pool<Db>,
    user: UserId,
) -> Option<i64> {
    match leaderboard {
        "coins" => Manager::coins_row_number(pool, user).await.ok().flatten(),
        "gems" => Manager::gems_row_number(pool, user).await.ok().flatten(),
        "eggplants" => Manager::eggplants_row_number(pool, user)
            .await
            .ok()
            .flatten(),
        "lottotickets" => Manager::lottotickets_row_number(pool, user)
            .await
            .ok()
            .flatten(),
        _ => None,
    }
}
