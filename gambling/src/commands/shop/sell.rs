use serenity::all::{
    CommandInteraction, Context, EditInteractionResponse, ResolvedOption, ResolvedValue, UserId,
};
use sqlx::prelude::FromRow;
use sqlx::types::Json;
use sqlx::{Database, Pool};
use zayden_core::{FormatNum, parse_options};

use crate::commands::shop::ShopManager;
use crate::models::{GamblingItem, ItemInventory};
use crate::shop::SALES_TAX;
use crate::{COIN, Coins, Error, Result, SHOP_ITEMS};

#[derive(FromRow)]
pub struct SellRow {
    pub id: i64,
    pub coins: i64,
    pub inventory: Option<Json<Vec<GamblingItem>>>,
}

impl SellRow {
    fn new(id: impl Into<UserId> + Send) -> Self {
        let id = id.into();

        Self {
            id: id.get() as i64,
            coins: 0,
            inventory: Some(Json(Vec::new())),
        }
    }
}

impl Coins for SellRow {
    fn coins(&self) -> i64 {
        self.coins
    }

    fn coins_mut(&mut self) -> &mut i64 {
        &mut self.coins
    }
}

impl ItemInventory for SellRow {
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

pub async fn sell<Db: Database, Manager: ShopManager<Db>>(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &Pool<Db>,
    options: Vec<ResolvedOption<'_>>,
) -> Result<()> {
    let mut options = parse_options(options);

    let Some(ResolvedValue::String(item)) = options.remove("item") else {
        unreachable!("item is required");
    };

    let Some(ResolvedValue::Integer(amount)) = options.remove("amount") else {
        unreachable!("amount is required")
    };

    if amount.is_negative() {
        return Err(Error::NegativeAmount);
    }

    let item = SHOP_ITEMS
        .get(item)
        .expect("Preset choices so item should always exist");
    let payment = ((item.coin_cost().unwrap() as f64) * (amount as f64) * (1.0 - SALES_TAX)) as i64;

    let mut row = match Manager::sell_row(pool, interaction.user.id).await.unwrap() {
        Some(row) => row,
        None => SellRow::new(interaction.user.id),
    };

    let inventory = row.inventory_mut();
    let inv_item = match inventory
        .iter_mut()
        .find(|inv_item| inv_item.item_id == item.id)
    {
        Some(item) => item,
        None => return Err(Error::ItemNotInInventory),
    };

    if inv_item.quantity < amount {
        return Err(Error::InsufficientItemQuantity(inv_item.quantity));
    }

    let quantity = row.edit_item_quantity(item.id, -amount).unwrap();

    *row.coins_mut() += payment;
    Manager::sell_save(pool, row).await.unwrap();

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content(format!(
                "You sold {} {item} for {} <:coin:{COIN}>\nYou now have {}.",
                amount.format(),
                payment.format(),
                quantity.format()
            )),
        )
        .await?;

    Ok(())
}
