use std::collections::HashMap;
use std::str::FromStr;

use chrono_tz::Tz;
use serenity::all::{CommandInteraction, Context, EditInteractionResponse, ResolvedValue};
use sqlx::{Database, Pool};

use crate::{Result, TimezoneManager};

use super::Command;

impl Command {
    pub async fn timezone<Db: Database, Manager: TimezoneManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        pool: &Pool<Db>,
        mut options: HashMap<&str, ResolvedValue<'_>>,
    ) -> Result<()> {
        interaction.defer_ephemeral(ctx).await.unwrap();

        let Some(ResolvedValue::String(region)) = options.remove("region") else {
            unreachable!("Region is required");
        };

        let tz = Tz::from_str(region).unwrap();

        Manager::save(pool, interaction.user.id, tz).await.unwrap();

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .content(format!("Your timezone has been set to {}", tz.name())),
            )
            .await
            .unwrap();

        Ok(())
    }
}
