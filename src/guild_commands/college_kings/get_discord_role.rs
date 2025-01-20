use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, EditInteractionResponse, Ready, ResolvedOption,
};
use sqlx::{PgPool, Postgres};
use zayden_core::SlashCommand;

use crate::{Error, Result};

const CONTENT: &str = "**How do I get my Discord role when I become a patreon?**
1. Make sure you're in the right Tier. If you made a “custom pledge,” instead of joining a Tier, you'll not be assigned any Discord roles.\n
2. After you confirm your payment amount, and Tier selection, you'll be taken to your creator's Welcome note. You can get started by clicking the **Connect to Discord** button.
3. You'll be taken to the App section of your __Profile settings__ page - click the **Connect** button to the right of the Discord app. Log in to your Discord account in the pop-up window that populates.\"
4. Now that your Patreon and Discord accounts are communicating, our integration will assign you the role tied to your Tier!
5. If you're still having trouble, please visit this website: <https://support.patreon.com/hc/en-us/articles/212052266-Get-my-Discord-role>";

pub struct GetDiscordRole;

#[async_trait]
impl SlashCommand<Error, Postgres> for GetDiscordRole {
    async fn run(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        _pool: &PgPool,
    ) -> Result<()> {
        interaction.defer_ephemeral(ctx).await.unwrap();

        interaction
            .edit_response(ctx, EditInteractionResponse::new().content(CONTENT))
            .await
            .unwrap();

        Ok(())
    }

    fn register(_ctx: &Context, _ready: &Ready) -> Result<CreateCommand> {
        let command =
            CreateCommand::new("get_discord_role").description("How do I get my Discord role");

        Ok(command)
    }
}
