use std::collections::HashSet;

use serenity::all::{
    ComponentInteraction, ComponentInteractionDataKind, Context, CreateInteractionResponse,
    EditThread, ForumTagId,
};

use crate::Result;

pub struct TagsComponent;

impl TagsComponent {
    pub async fn add(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
        let mut tag_ids = match &interaction.data.kind {
            ComponentInteractionDataKind::StringSelect { values } => values
                .iter()
                .map(|x| x.parse::<u64>().unwrap())
                .map(|id| ForumTagId::new(id))
                .collect::<HashSet<_>>(),
            _ => unreachable!("Expected string select"),
        };

        let mut channel = interaction
            .channel_id
            .to_channel(ctx)
            .await
            .unwrap()
            .guild()
            .unwrap();

        tag_ids.extend(channel.applied_tags.iter().copied());

        channel
            .edit_thread(ctx, EditThread::new().applied_tags(tag_ids))
            .await
            .unwrap();

        interaction
            .create_response(ctx, CreateInteractionResponse::Acknowledge)
            .await
            .unwrap();

        Ok(())
    }

    pub async fn remove(ctx: &Context, interaction: &ComponentInteraction) -> Result<()> {
        let selected_ids = match &interaction.data.kind {
            ComponentInteractionDataKind::StringSelect { values } => values
                .iter()
                .map(|x| x.parse::<u64>().unwrap())
                .map(ForumTagId::new)
                .collect::<HashSet<_>>(),
            _ => unreachable!("Expected string select"),
        };

        let mut channel = interaction
            .channel_id
            .to_channel(ctx)
            .await
            .unwrap()
            .guild()
            .unwrap();

        let new_tag_ids = channel
            .applied_tags
            .iter()
            .copied()
            .filter(|tag_id| !selected_ids.contains(tag_id))
            .collect::<HashSet<_>>();

        channel
            .edit_thread(ctx, EditThread::new().applied_tags(new_tag_ids))
            .await
            .unwrap();

        interaction
            .create_response(ctx, CreateInteractionResponse::Acknowledge)
            .await
            .unwrap();

        Ok(())
    }
}
