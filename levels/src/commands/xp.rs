use serenity::all::{
    CommandInteraction, Context, CreateEmbed, EditInteractionResponse, ResolvedOption,
    ResolvedValue,
};
use sqlx::{Database, Pool};
use zayden_core::parse_options;

use crate::{LevelsManager, LevelsRow};

use super::Commands;

impl Commands {
    pub async fn xp<Db: Database, Manager: LevelsManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &Pool<Db>,
    ) {
        let mut options = parse_options(options);

        match options.remove("ephemeral") {
            Some(ResolvedValue::Boolean(true)) => interaction.defer_ephemeral(ctx).await.unwrap(),
            _ => interaction.defer(ctx).await.unwrap(),
        }

        let row = Manager::xp_row(pool, interaction.user.id)
            .await
            .unwrap()
            .unwrap_or_default();

        let embed = CreateEmbed::default().title("XP").description(format!(
            "Current XP: {}\nLevel: {}\nTotal XP: {}",
            row.xp(),
            row.level(),
            row.total_xp()
        ));

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed))
            .await
            .unwrap();
    }
}
