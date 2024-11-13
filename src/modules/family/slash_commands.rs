use async_trait::async_trait;
use charming::{
    element::{Color, LineStyle},
    series::{Graph, GraphData, GraphLayout},
    Chart, ImageFormat, ImageRenderer,
};
use family::commands::{
    Adopt, Block, Children, FamilyCommand, Marry, Parents, Partner, Relationship, Siblings, Tree,
    Unblock,
};
use serenity::all::{
    ButtonStyle, CommandInteraction, Context, CreateAttachment, CreateButton, CreateCommand,
    CreateEmbed, EditInteractionResponse, Mentionable, Ready,
};
use sqlx::Postgres;
use zayden_core::SlashCommand;

use crate::{sqlx_lib::PostgresPool, Error, Result};

use super::FamilyTable;

pub struct AdoptCommand;

#[async_trait]
impl SlashCommand<Error> for AdoptCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction.defer(ctx).await?;

        let pool = PostgresPool::get(ctx).await;

        let user_id = Adopt::run::<Postgres, FamilyTable>(ctx, interaction, &pool).await?;

        interaction
            .edit_response(
                &ctx,
                EditInteractionResponse::new()
                    .content(format!(
                        "{}, {} wants to adopt you! Do you accept?",
                        user_id.mention(),
                        interaction.user.mention()
                    ))
                    .button(
                        CreateButton::new("adopt_accept")
                            .label("Yes")
                            .style(ButtonStyle::Success),
                    )
                    .button(
                        CreateButton::new("adopt_decline")
                            .label("No")
                            .style(ButtonStyle::Danger),
                    ),
            )
            .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(Adopt::register())
    }
}

pub struct BlockCommand;

#[async_trait]
impl SlashCommand<Error> for BlockCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction.defer_ephemeral(ctx).await?;

        let pool = PostgresPool::get(ctx).await;

        Block::run::<Postgres, FamilyTable>(ctx, interaction, &pool).await?;

        interaction
            .edit_response(ctx, EditInteractionResponse::new().content("User blocked."))
            .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(Block::register())
    }
}

pub struct UnblockCommand;

#[async_trait]
impl SlashCommand<Error> for UnblockCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction.defer_ephemeral(ctx).await?;

        let pool = PostgresPool::get(ctx).await;

        Unblock::run::<Postgres, FamilyTable>(ctx, interaction, &pool).await?;

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().content("User unblocked."),
            )
            .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(Unblock::register())
    }
}

pub struct ChildrenCommand;

#[async_trait]
impl SlashCommand<Error> for ChildrenCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction.defer(ctx).await?;

        let pool = PostgresPool::get(ctx).await;

        let (user_id, children) =
            Children::run::<Postgres, FamilyTable>(ctx, interaction, &pool).await?;

        let children_plural = if children.len() == 1 {
            "child"
        } else {
            "children"
        };

        let desc = if user_id == interaction.user.id {
            format!(
                "You have {} {}:\n{}",
                children.len(),
                children_plural,
                children.join("\n")
            )
        } else {
            format!(
                "{} has {} {}:\n{}",
                user_id.mention(),
                children.len(),
                children_plural,
                children.join("\n")
            )
        };

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().embed(CreateEmbed::new().description(desc)),
            )
            .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(Children::register())
    }
}

pub struct MarryCommand;

#[async_trait]
impl SlashCommand<Error> for MarryCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction.defer(ctx).await?;

        let pool = PostgresPool::get(ctx).await;

        let target_id = Marry::run::<Postgres, FamilyTable>(ctx, interaction, &pool).await?;

        interaction
            .edit_response(
                &ctx,
                EditInteractionResponse::new()
                    .content(format!(
                        "{}, it would make {} really happy if you would marry them. Do you accept?",
                        target_id.mention(),
                        interaction.user.mention()
                    ))
                    .button(
                        CreateButton::new("marry_accept")
                            .label("Yes")
                            .style(ButtonStyle::Success),
                    )
                    .button(
                        CreateButton::new("marry_decline")
                            .label("No")
                            .style(ButtonStyle::Danger),
                    ),
            )
            .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(Marry::register())
    }
}

pub struct ParentsCommand;

#[async_trait]
impl SlashCommand<Error> for ParentsCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction.defer(ctx).await?;

        let pool = PostgresPool::get(ctx).await;

        let (user_id, parents) =
            Parents::run::<Postgres, FamilyTable>(ctx, interaction, &pool).await?;

        let parents_plural = if parents.len() == 1 {
            "parent"
        } else {
            "parents"
        };

        let desc = if user_id == interaction.user.id {
            format!(
                "You have {} {}:\n{}",
                parents.len(),
                parents_plural,
                parents.join("\n")
            )
        } else {
            format!(
                "{} has {} {}:\n{}",
                user_id.mention(),
                parents.len(),
                parents_plural,
                parents.join("\n")
            )
        };

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().embed(CreateEmbed::new().description(desc)),
            )
            .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(Parents::register())
    }
}

pub struct PartnersCommand;

#[async_trait]
impl SlashCommand<Error> for PartnersCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction.defer(ctx).await?;

        let pool = PostgresPool::get(ctx).await;

        let (user_id, partners) =
            Partner::run::<Postgres, FamilyTable>(ctx, interaction, &pool).await?;

        let partners_plural = if partners.len() == 1 {
            "partner"
        } else {
            "partners"
        };

        let desc = if user_id == interaction.user.id {
            format!(
                "You have {} {}:\n{}",
                partners.len(),
                partners_plural,
                partners.join("\n")
            )
        } else {
            format!(
                "{} has {} {}:\n{}",
                user_id.mention(),
                partners.len(),
                partners_plural,
                partners.join("\n")
            )
        };

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().embed(CreateEmbed::new().description(desc)),
            )
            .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(Partner::register())
    }
}

pub struct RelationshipCommand;

#[async_trait]
impl SlashCommand<Error> for RelationshipCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction.defer(ctx).await?;

        let pool = PostgresPool::get(ctx).await;

        let res = Relationship::run::<Postgres, FamilyTable>(ctx, interaction, &pool).await?;

        let embed = if res.other_id == interaction.user.id {
            CreateEmbed::new().description(format!(
                "{} is your {}",
                res.user_id.mention(),
                res.relationship
            ))
        } else {
            CreateEmbed::new().description(format!(
                "{} is {}'s {}",
                res.other_id.mention(),
                res.user_id.mention(),
                res.relationship
            ))
        };

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed))
            .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(Relationship::register())
    }
}

pub struct SiblingsCommand;

#[async_trait]
impl SlashCommand<Error> for SiblingsCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction.defer(ctx).await?;

        let pool = PostgresPool::get(ctx).await;

        let (user_id, siblings) =
            Siblings::run::<Postgres, FamilyTable>(ctx, interaction, &pool).await?;

        let siblings_plural = if siblings.len() == 1 {
            "sibling"
        } else {
            "siblings"
        };

        let desc = if user_id == interaction.user.id {
            format!(
                "You have {} {}:\n{}",
                siblings.len(),
                siblings_plural,
                siblings.join("\n")
            )
        } else {
            format!(
                "{} has {} {}:\n{}",
                user_id.mention(),
                siblings.len(),
                siblings_plural,
                siblings.join("\n")
            )
        };

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new().embed(CreateEmbed::new().description(desc)),
            )
            .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(Siblings::register())
    }
}

pub struct TreeCommand;

fn render_graph(data: GraphData) -> Result<Vec<u8>> {
    let chart = Chart::new().series(
        Graph::new()
            .layout(GraphLayout::None)
            .line_style(
                LineStyle::new()
                    .color(Color::Value(String::from("#ffffff")))
                    .width(10),
            )
            .data(data),
    );
    let mut renderer = ImageRenderer::new(1920, 1080);
    Ok(renderer.render_format(ImageFormat::Png, &chart)?)
}

#[async_trait]
impl SlashCommand<Error> for TreeCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction.defer(ctx).await?;

        let pool = PostgresPool::get(ctx).await;

        let tree_data = Tree::run::<Postgres, FamilyTable>(ctx, interaction, &pool).await?;

        let graph_data = render_graph(tree_data)?;

        interaction
            .edit_response(
                ctx,
                EditInteractionResponse::new()
                    .new_attachment(CreateAttachment::bytes(graph_data, "graph.png")),
            )
            .await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        Ok(Tree::register())
    }
}
