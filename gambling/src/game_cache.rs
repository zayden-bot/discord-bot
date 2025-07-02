use std::collections::HashMap;
use std::sync::RwLockReadGuard;

use chrono::{DateTime, Duration, Utc};
use serenity::all::{Context, UserId};
use serenity::prelude::{TypeMap, TypeMapKey};

use crate::{Error, Result};

pub struct GameCache(HashMap<UserId, DateTime<Utc>>);

impl GameCache {
    pub fn get<'a>(data: &'a RwLockReadGuard<'_, TypeMap>) -> Option<&'a Self> {
        data.get::<GameCache>()
    }

    pub async fn can_play(ctx: &Context, id: impl Into<UserId>) -> Result<()> {
        let data = ctx.data.read().await;
        match data.get::<GameCache>() {
            Some(cache) => cache.validate_cooldown(id),
            None => Ok(()),
        }
    }

    fn validate_cooldown(&self, id: impl Into<UserId>) -> Result<()> {
        let id = id.into();

        if let Some(last_played) = self.0.get(&id) {
            let cooldown_over = *last_played + Duration::seconds(5);

            if cooldown_over >= Utc::now() {
                return Err(Error::Cooldown(cooldown_over.timestamp()));
            }
        }

        Ok(())
    }

    pub async fn update(ctx: &Context, id: impl Into<UserId>) {
        let mut data = ctx.data.write().await;
        let cache = data
            .entry::<GameCache>()
            .or_insert(GameCache(HashMap::new()));

        cache.0.insert(id.into(), Utc::now());
    }
}

impl TypeMapKey for GameCache {
    type Value = GameCache;
}
