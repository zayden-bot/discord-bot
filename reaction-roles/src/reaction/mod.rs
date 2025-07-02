use serenity::all::{Context, Reaction};
use sqlx::Pool;

use crate::{Error, ReactionRolesManager, Result};

pub struct ReactionRoleReaction;

impl ReactionRoleReaction {
    pub async fn reaction_add<Db, Manager>(
        ctx: &Context,
        reaction: &Reaction,
        pool: &Pool<Db>,
    ) -> Result<()>
    where
        Db: sqlx::Database,
        Manager: ReactionRolesManager<Db>,
    {
        let emoji_string = reaction.emoji.to_string();
        let reaction_role = Manager::get_row(pool, reaction.message_id, &emoji_string)
            .await
            .unwrap();

        if let Some(reaction_role) = reaction_role {
            let member = reaction.member.as_ref().ok_or(Error::MissingGuildId)?;

            member.add_role(ctx, reaction_role.role_id()).await.unwrap();
        }

        Ok(())
    }

    pub async fn reaction_remove<Db, Manager>(
        ctx: &Context,
        reaction: &Reaction,
        pool: &Pool<Db>,
    ) -> Result<()>
    where
        Db: sqlx::Database,
        Manager: ReactionRolesManager<Db>,
    {
        let reaction_role =
            Manager::get_row(pool, reaction.message_id, &reaction.emoji.to_string())
                .await
                .unwrap();

        if let Some(reaction_role) = reaction_role {
            let member = reaction
                .guild_id
                .ok_or(Error::MissingGuildId)?
                .member(ctx, reaction.user_id.unwrap())
                .await
                .unwrap();

            member
                .remove_role(ctx, reaction_role.role_id())
                .await
                .unwrap();
        }

        Ok(())
    }
}
