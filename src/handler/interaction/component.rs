use serenity::all::{
    ComponentInteraction, Context, CreateInteractionResponse, EditInteractionResponse,
};
use sqlx::{PgPool, Postgres};
use suggestions::Suggestions;
use zayden_core::ErrorResponse;

use crate::handler::Handler;
use crate::modules::destiny2::lfg::{LfgMessageTable, LfgPostTable};
use crate::modules::ticket::Ticket;
use crate::{Error, Result};

impl Handler {
    pub async fn interaction_component(
        ctx: &Context,
        interaction: &ComponentInteraction,
        pool: &PgPool,
    ) -> Result<()> {
        println!(
            "{} ran component: {}",
            interaction.user.name, interaction.data.custom_id
        );

        let result = match interaction.data.custom_id.as_str() {
            // region: Lfg
            "lfg_join" => lfg::PostComponents::join::<Postgres, LfgPostTable, LfgMessageTable>(
                ctx,
                interaction,
                pool,
            )
            .await
            .map_err(Error::from),
            "lfg_leave" => lfg::PostComponents::leave::<Postgres, LfgPostTable, LfgMessageTable>(
                ctx,
                interaction,
                pool,
            )
            .await
            .map_err(Error::from),
            "lfg_alternative" => lfg::PostComponents::alternative::<
                Postgres,
                LfgPostTable,
                LfgMessageTable,
            >(ctx, interaction, pool)
            .await
            .map_err(Error::from),
            "lfg_settings" => {
                lfg::PostComponents::settings::<Postgres, LfgPostTable>(ctx, interaction, pool)
                    .await
                    .map_err(Error::from)
            }

            "lfg_edit" => {
                lfg::SettingsComponents::edit::<Postgres, LfgPostTable>(ctx, interaction, pool)
                    .await
                    .map_err(Error::from)
            }
            "lfg_copy" => {
                lfg::SettingsComponents::copy::<Postgres, LfgPostTable>(ctx, interaction, pool)
                    .await
                    .map_err(Error::from)
            }
            "lfg_kick" => {
                lfg::SettingsComponents::kick::<Postgres, LfgPostTable>(ctx, interaction, pool)
                    .await
                    .map_err(Error::from)
            }
            "lfg_kick_menu" => lfg::KickComponent::run::<Postgres, LfgPostTable, LfgMessageTable>(
                ctx,
                interaction,
                pool,
            )
            .await
            .map_err(Error::from),
            "lfg_delete" => {
                lfg::SettingsComponents::delete::<Postgres, LfgPostTable>(ctx, interaction, pool)
                    .await?;

                interaction
                    .create_response(ctx, CreateInteractionResponse::Acknowledge)
                    .await
                    .unwrap();

                Ok(())
            }
            // "lfg_tags_add" => Lfg::tags_add(ctx, interaction).await,
            // "lfg_tags_remove" => Lfg::tags_remove(ctx, interaction).await,
            // endregion
            "suggestions_accept" | "suggestions_added" | "accept" => {
                Suggestions::components(ctx, interaction, true).await;
                Ok(())
            }
            "suggestions_reject" | "reject" => {
                Suggestions::components(ctx, interaction, false).await;
                Ok(())
            }

            //region: Ticket
            "ticket_create" | "support_ticket" => Ticket::ticket_create(ctx, interaction).await,
            "support_close" => Ticket::support_close(ctx, interaction).await,
            "support_faq" => Ticket::support_faq(ctx, interaction, pool).await,
            //endregion: Ticket
            _ => unimplemented!("Component not implemented: {}", interaction.data.custom_id),
        };

        if let Err(e) = result {
            let msg = e.to_response();

            let _ = interaction.defer_ephemeral(ctx).await;

            interaction
                .edit_response(ctx, EditInteractionResponse::new().content(msg))
                .await
                .unwrap();
        }

        Ok(())
    }
}
