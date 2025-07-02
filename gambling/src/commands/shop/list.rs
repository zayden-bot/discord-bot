use std::time::Duration;

use async_trait::async_trait;
use futures::StreamExt;
use serenity::all::{
    ButtonStyle, CommandInteraction, Context, CreateActionRow, CreateButton, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse,
    ReactionType, ResolvedOption, ResolvedValue, UserId,
};
use sqlx::{Database, FromRow, Pool, any::AnyQueryResult, types::Json};
use zayden_core::FormatNum;

use crate::{
    COIN, Coins, GamblingItem, ItemInventory, Result, SHOP_ITEMS, ShopPage,
    commands::shop::ShopManager, shop::SALES_TAX,
};

#[async_trait]
pub trait ListManager<Db: Database> {
    async fn save(pool: &Pool<Db>, row: ListRow) -> sqlx::Result<AnyQueryResult>;
}

#[derive(FromRow)]
pub struct ListRow {
    pub id: i64,
    pub coins: i64,
    pub inventory: Option<Json<Vec<GamblingItem>>>,
}

impl Coins for ListRow {
    fn coins(&self) -> i64 {
        self.coins
    }

    fn coins_mut(&mut self) -> &mut i64 {
        &mut self.coins
    }
}

impl ListRow {
    fn new(id: impl Into<UserId> + Send) -> Self {
        let id = id.into();

        Self {
            id: id.get() as i64,
            coins: 0,
            inventory: Some(Json(Vec::new())),
        }
    }
}

impl ItemInventory for ListRow {
    fn inventory(&self) -> &[GamblingItem] {
        match self.inventory.as_ref() {
            Some(vec_ref) => &vec_ref.0,
            None => &[],
        }
    }

    fn inventory_mut(&mut self) -> &mut Vec<GamblingItem> {
        self.inventory.get_or_insert_with(|| Json(Vec::new()))
    }
}

pub async fn list<Db: Database, Manager: ShopManager<Db>>(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &Pool<Db>,
    mut options: Vec<ResolvedOption<'_>>,
) -> Result<()> {
    let page = match options.pop().map(|opt| opt.value) {
        Some(ResolvedValue::String(page)) => page.parse().unwrap(),
        _ => ShopPage::Item,
    };

    let row = match Manager::list_row(pool, interaction.user.id).await.unwrap() {
        Some(row) => row,
        None => ListRow::new(interaction.user.id),
    };

    let embed = create_embed(page, &row);

    let left_arrow = ReactionType::Unicode(String::from("⬅️"));
    let right_arrow = ReactionType::Unicode(String::from("➡️"));

    let prev = CreateButton::new("shop_prev")
        .emoji(left_arrow)
        .style(ButtonStyle::Secondary);
    let next = CreateButton::new("shop_next")
        .emoji(right_arrow)
        .style(ButtonStyle::Secondary);

    let msg = interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new()
                .embed(embed)
                .components(vec![CreateActionRow::Buttons(vec![prev, next])]),
        )
        .await?;

    let mut stream = msg
        .await_component_interactions(ctx)
        .timeout(Duration::from_secs(120))
        .stream();

    while let Some(interaction) = stream.next().await {
        let title = interaction
            .message
            .embeds
            .first()
            .and_then(|embed| embed.title.as_deref());

        let (embed, components) = if interaction.data.custom_id == "shop_next" {
            shop(&row, title, 1)
        } else {
            shop(&row, title, -1)
        };

        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .embed(embed)
                        .components(components),
                ),
            )
            .await
            .unwrap();
    }

    interaction
        .edit_response(ctx, EditInteractionResponse::new().components(Vec::new()))
        .await?;

    Ok(())
}

fn shop(
    row: &ListRow,
    title: Option<&str>,
    page_change: i8,
) -> (CreateEmbed, Vec<CreateActionRow>) {
    let current_cat = title
        .map(|title| title.strip_suffix(" Shop").unwrap().parse().unwrap())
        .unwrap_or(ShopPage::Item);

    let category_idx = ShopPage::pages()
        .iter()
        .position(|cat| *cat == current_cat)
        .unwrap() as i8;

    let category = ShopPage::pages()
        .get(usize::try_from(category_idx + page_change).unwrap_or_default())
        .copied()
        .unwrap_or(ShopPage::Item);

    let embed = create_embed(category, row);

    let left_arrow = ReactionType::Unicode(String::from("⬅️"));
    let right_arrow = ReactionType::Unicode(String::from("➡️"));

    let prev = CreateButton::new("shop_prev")
        .emoji(left_arrow)
        .style(ButtonStyle::Secondary);
    let next = CreateButton::new("shop_next")
        .emoji(right_arrow)
        .style(ButtonStyle::Secondary);

    (embed, vec![CreateActionRow::Buttons(vec![prev, next])])
}

fn create_embed(category: ShopPage, row: &ListRow) -> CreateEmbed {
    let inv = row.inventory();

    let items = SHOP_ITEMS
        .iter()
        .filter(|item| item.category == category)
        .map(|item| {
            let costs = item
                .cost
                .iter()
                .filter_map(|x| x.as_ref())
                .map(|(cost, currency)| format!("`{}` {}", cost.format(), currency))
                .collect::<Vec<_>>();

            let mut s = format!("**{item}**");

            if !item.description.is_empty() {
                s.push('\n');
                s.push_str(item.description);
            }

            s.push_str(&format!(
                "\nOwned: `{}`\nCost:",
                inv.iter()
                    .find(|inv_item| inv_item.item_id == item.id)
                    .map(|item| item.quantity)
                    .unwrap_or_default()
            ));

            if costs.len() == 1 {
                s.push(' ');
                s.push_str(&costs.join(""));
            } else {
                s.push('\n');
                s.push_str(&costs.join("\n"));
            }

            s
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    let desc = format!(
        "Sales tax: {}%\nYour coins: {}  <:coin:{COIN}>\n--------------------\n{items}\n--------------------\nBuy with `/shop buy <item> <amount>`\nSell with `/shop sell <item> <amount>`",
        SALES_TAX * 100.0,
        row.coins_str()
    );

    CreateEmbed::new()
        .title(format!("{} Shop", category))
        .description(desc)
}
