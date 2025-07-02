use futures::{StreamExt, TryStreamExt};
use serenity::all::{
    ComponentInteraction, ComponentInteractionDataKind, Context, CreateActionRow, CreateEmbed,
    CreateInputText, CreateInteractionResponse, CreateInteractionResponseMessage, CreateModal,
    EditThread, InputTextStyle,
};
use sqlx::{Database, Pool};

use crate::{Error, Result, TicketGuildManager};

pub struct TicketComponent;

impl TicketComponent {
    pub async fn ticket_create(
        ctx: &Context,
        interaction: &ComponentInteraction,
        compnents: impl IntoIterator<Item = CreateInputText>,
    ) -> Result<()> {
        let issue_input = CreateInputText::new(InputTextStyle::Paragraph, "Issue", "issue")
            .placeholder("Describe the issue you're experiencing");

        let modal = CreateModal::new("create_ticket", "Ticket").components(
            vec![issue_input]
                .into_iter()
                .chain(compnents)
                .map(CreateActionRow::InputText)
                .collect(),
        );

        interaction
            .create_response(&ctx, CreateInteractionResponse::Modal(modal))
            .await?;

        Ok(())
    }

    pub async fn support_close(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
        let channel = interaction.channel.as_ref().unwrap();

        let new_channel_name: String =
            format!("{} - {}", "[Closed]", channel.name.as_ref().unwrap())
                .chars()
                .take(100)
                .collect();

        interaction
            .channel_id
            .edit_thread(ctx, EditThread::new().name(new_channel_name).archived(true))
            .await?;

        interaction
            .create_response(ctx, CreateInteractionResponse::Acknowledge)
            .await?;

        Ok(())
    }

    pub async fn support_faq<Db: Database, GuildManager: TicketGuildManager<Db>>(
        ctx: &Context,
        interaction: &ComponentInteraction,
        pool: &Pool<Db>,
    ) -> Result<()> {
        let guild_id = interaction.guild_id.ok_or(Error::MissingGuildId)?;

        let index = match &interaction.data.kind {
            ComponentInteractionDataKind::StringSelect { values } => {
                values[0].parse::<usize>().unwrap()
            }
            _ => unreachable!("Invalid interaction data kind"),
        };

        let faq_channel_id = GuildManager::get(pool, guild_id)
            .await
            .unwrap()
            .unwrap()
            .faq_channel_id()
            .unwrap();

        let message = faq_channel_id
            .messages_iter(ctx)
            .skip(index)
            .boxed()
            .try_next()
            .await
            .unwrap()
            .unwrap();

        let mut parts: Vec<&str> = message.content.split("**").collect();
        let description = parts.pop().unwrap().trim();
        let title = parts.join("");

        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().embed(
                        CreateEmbed::new()
                            .title(title.trim())
                            .description(description),
                    ),
                ),
            )
            .await?;

        Ok(())
    }
}
