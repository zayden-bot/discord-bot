pub mod post;
pub mod timezone_manager;

use async_trait::async_trait;
pub use post::{PostBuilder, PostManager, PostRow};
use serenity::all::UserId;
use sqlx::{Database, Pool};
pub use timezone_manager::TimezoneManager;

use crate::{Error, Result};

#[async_trait]
pub trait Savable<Db: Database, T> {
    async fn save(pool: &Pool<Db>, item: T) -> sqlx::Result<Db::QueryResult>;
}

pub trait Leave {
    fn fireteam_mut(&mut self) -> &mut Vec<i64>;

    fn alternatives_mut(&mut self) -> &mut Vec<i64>;

    fn leave(&mut self, user: impl Into<UserId>) {
        let user = user.into().get() as i64;

        self.fireteam_mut().retain(|&id| id != user);
        self.alternatives_mut().retain(|&id| id != user);
    }
}

pub trait Join: Leave {
    fn fireteam_size(&self) -> i16;

    fn fireteam(&self) -> impl Iterator<Item = UserId>;

    fn fireteam_len(&self) -> i16;

    fn alternatives(&self) -> impl Iterator<Item = UserId>;

    fn is_full(&self) -> bool {
        self.fireteam_len() == self.fireteam_size()
    }

    fn join(&mut self, user: impl Into<UserId>, alternative: bool) -> Result<()> {
        let user = user.into();

        if !alternative && self.fireteam().any(|id| id == user) {
            return Err(Error::AlreadyJoined);
        }

        if alternative && self.alternatives().any(|id| id == user) {
            return Err(Error::AlreadyJoined);
        }

        if !alternative && self.is_full() {
            return Err(Error::FireteamFull);
        }

        self.leave(user);

        if alternative {
            self.alternatives_mut().push(user.get() as i64);
        } else {
            self.fireteam_mut().push(user.get() as i64);
        }

        Ok(())
    }
}
