use family::components::{adopt, marry};
use serenity::all::{ComponentInteraction, Context, EditInteractionResponse, Mentionable};
use sqlx::Postgres;

use crate::{sqlx_lib::PostgresPool, Result};

use super::FamilyTable;

pub struct AdoptComponent;

impl AdoptComponent {
    pub async fn accept(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
        interaction.defer(ctx).await?;

        let pool = PostgresPool::get(ctx).await;

        let parent_id = adopt::accept::<Postgres, FamilyTable>(interaction, &pool).await?;

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .content(format!(
                        "Pleased to introduce {} as your new parent, {}!",
                        parent_id.mention(),
                        interaction.user.mention()
                    ))
                    .components(Vec::new()),
            )
            .await?;

        Ok(())
    }

    pub async fn decline(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
        interaction.defer(ctx).await?;

        adopt::decline(interaction).await?;

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .content(format!("Sorry, {} said no.", interaction.user.mention()))
                    .components(Vec::new()),
            )
            .await?;

        Ok(())
    }
}

pub struct MarryComponent;

impl MarryComponent {
    pub async fn accept(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
        interaction.defer(ctx).await?;

        let pool = PostgresPool::get(ctx).await;

        marry::accept::<Postgres, FamilyTable>(interaction, &pool).await?;

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .content(format!(
                        "Congratulations! {} and {} you are now married.",
                        interaction.message.mentions[0].mention(),
                        interaction.message.mentions[1].mention()
                    ))
                    .components(Vec::new()),
            )
            .await?;

        Ok(())
    }

    pub async fn decline(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
        interaction.defer(ctx).await?;

        marry::decline(interaction).await?;

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .content(format!("Sorry, {} said no.", interaction.user.mention()))
                    .components(Vec::new()),
            )
            .await?;

        Ok(())
    }
}
