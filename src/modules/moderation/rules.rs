use std::path::PathBuf;
use std::str::FromStr;

use async_trait::async_trait;
use serenity::all::{
    ChannelId, Colour, CommandInteraction, CreateEmbed, EditMessage, MessageId, Ready,
};
use serenity::builder::CreateCommand;
use serenity::model::Permissions;
use serenity::prelude::Context;
use zayden_core::SlashCommand;

use crate::utils::message_response;
use crate::{Error, Result};

const CHANNEL_ID: ChannelId = ChannelId::new(747430712617074718);
const MESSAGE_ID: MessageId = MessageId::new(788539168980336701);

pub struct RulesCommand;

#[async_trait]
impl SlashCommand<Error> for RulesCommand {
    async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction.defer_ephemeral(ctx).await?;

        let rules_path = PathBuf::from_str("messages").unwrap().join("rules.md");

        let rules = tokio::fs::read_to_string(rules_path).await?;
        let fields = rules.split("\r\n\r\n").map(|rule| {
            let mut lines = rule.lines();
            let title = lines.next().unwrap();
            let description = lines.collect::<Vec<&str>>().join("\n");
            (title, description, false)
        });

        let embed = CreateEmbed::new().colour(Colour::from_rgb(255, 0, 0)).title("College Kings Server Rules").description("The below rules are a truncated version of the rules found in the [Code of Conduct](https://gist.github.com/KiloOscarSix/201a919b5650e511f11287c0a9c4c2fc).").fields(fields);

        let mut message = CHANNEL_ID.message(ctx, MESSAGE_ID).await?;
        message.edit(ctx, EditMessage::new().embed(embed)).await?;

        message_response(ctx, interaction, "The rules have been sent.").await?;

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command = CreateCommand::new("rules")
            .description("Display the server rules")
            .default_member_permissions(Permissions::MODERATE_MEMBERS);

        Ok(command)
    }
}
