use serenity::all::{ComponentInteraction, Context, CreateInputText, InputTextStyle};
use sqlx::{PgPool, Postgres};
use ticket::TicketComponent;

use crate::sqlx_lib::GuildTable;
use crate::Result;

pub async fn support_close(ctx: &Context, component: &ComponentInteraction) -> Result<()> {
    TicketComponent::support_close(ctx, component).await?;

    Ok(())
}

pub async fn support_faq(
    ctx: &Context,
    component: &ComponentInteraction,
    pool: &PgPool,
) -> Result<()> {
    TicketComponent::support_faq::<Postgres, GuildTable>(ctx, component, pool).await?;

    Ok(())
}

pub async fn support_ticket(ctx: &Context, component: &ComponentInteraction) -> Result<()> {
    let version =
        CreateInputText::new(InputTextStyle::Short, "Version", "version").placeholder("1.0.0");

    let additional = CreateInputText::new(
        InputTextStyle::Paragraph,
        "Additional Information",
        "additional",
    )
    .placeholder("Please provide any additional information that may help us assist you.");

    TicketComponent::support_ticket(ctx, component, vec![version, additional]).await?;

    Ok(())
}
