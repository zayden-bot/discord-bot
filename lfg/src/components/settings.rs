use serenity::all::{
    ComponentInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use sqlx::{Database, Pool};

use crate::templates::{DefaultTemplate, Template};
use crate::{Error, PostManager, Result};

use super::Components;

impl Components {
    pub async fn settings<Db: Database, Manager: PostManager<Db>>(
        ctx: &Context,
        interaction: &ComponentInteraction,
        pool: &Pool<Db>,
    ) -> Result<()> {
        let owner = match Manager::owner(pool, interaction.channel_id).await {
            Ok(owner) => owner,
            Err(sqlx::Error::RowNotFound) => interaction.user.id,
            Err(e) => panic!("{e:?}"),
        };

        if interaction.user.id != owner {
            return Err(Error::PermissionDenied(owner));
        }

        let main_row = DefaultTemplate::main_row();
        let settings_row = DefaultTemplate::settings_row();

        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .components(vec![main_row, settings_row]),
                ),
            )
            .await
            .unwrap();

        Ok(())
    }
}
