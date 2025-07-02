use serenity::all::{
    CommandInteraction, Context, CreateEmbed, EditInteractionResponse, ResolvedOption,
    ResolvedValue,
};
use sqlx::{Database, Pool};
use zayden_core::parse_options;

use crate::{level_up_xp, LevelsManager, LevelsRow};

use super::Commands;

impl Commands {
    pub async fn rank<Db: Database, Manager: LevelsManager<Db>>(
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

        let user = match options.remove("user") {
            Some(ResolvedValue::User(user, _)) => user,
            _ => &interaction.user,
        };

        let row = Manager::rank_row(pool, user.id)
            .await
            .unwrap()
            .unwrap_or_default();

        let level = row.level();
        let xp_for_next_level = level_up_xp(level);

        let user_rank = match Manager::user_rank(pool, user.id).await.unwrap() {
            Some(rank) => format!("{rank}"),
            None => String::from("N/A"),
        };

        let xp = row.xp();

        let embed = CreateEmbed::new()
            .title(format!("XP stats for {}", user.name))
            .description(format!(
                "Rank: #{user_rank}\nLevel: {level}\nXP: {xp}/{xp_for_next_level} ({}%)",
                (xp as f32 / xp_for_next_level as f32 * 100.0).round()
            ));

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed))
            .await
            .unwrap();
    }
}
