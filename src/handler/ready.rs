use lfg::close_old_posts;
// use futures::future;
use serenity::all::{Context, OnlineStatus, Ready};
use sqlx::{PgPool, Postgres};

use crate::Result;
use crate::handler::Handler;
use crate::modules::destiny2::lfg::LfgPostTable;
// use crate::modules;

impl Handler {
    pub async fn ready(ctx: &Context, ready: Ready, pool: &PgPool) -> Result<()> {
        println!("{} is connected!", ready.user.name);

        ctx.set_presence(None, OnlineStatus::Online);

        for command in ctx.http.get_global_commands().await.unwrap() {
            ctx.http.delete_global_command(command.id).await.unwrap();
        }

        close_old_posts::<Postgres, LfgPostTable>(ctx, pool).await;

        Ok(())
    }
}
