pub mod user_levels;

use chrono::{TimeDelta, Utc};
use lazy_static::lazy_static;
use rand::Rng;
use serenity::all::{ChannelId, Context, Message, RoleId};
use std::collections::HashMap;

use user_levels::{get_user_level_data, update_user_level_data};

use crate::{Error, Result};

const BLOCKED_CHANNEL_IDS: [ChannelId; 1] = [ChannelId::new(776139754408247326)];

lazy_static! {
    static ref LEVEL_ROLES: HashMap<i32, RoleId> = {
        let mut map = HashMap::new();
        map.insert(5, RoleId::new(787443819024220210)); // New Fan | Level 5
        map.insert(10, RoleId::new(787445571539304510)); // Active Fan | Level 10
        map.insert(20, RoleId::new(787445900992577556)); // Big Fan | Level 20
        map.insert(40, RoleId::new(787446715057831976)); // Super Fan | Level 40
        map.insert(60, RoleId::new(787447090728796191)); // Mega Fan | Level 60
        map.insert(80, RoleId::new(787447252783202326)); // Ultra Fan | Level 80
        map
    };
}

pub async fn run(ctx: &Context, msg: &Message) -> Result<Option<()>> {
    if msg.guild_id.is_none() {
        return Ok(None);
    }

    if BLOCKED_CHANNEL_IDS.contains(&msg.channel_id) {
        return Ok(None);
    }

    let level_data = get_user_level_data(msg.author.id.get()).await?;

    if level_data.last_xp
        >= (Utc::now().naive_utc() - TimeDelta::try_minutes(1).ok_or_else(|| Error::TimeDelta)?)
    {
        return Ok(None);
    }

    let mut level = 0;
    let rand_xp = rand::thread_rng().gen_range(15..25);
    let total_xp = level_data.total_xp + rand_xp;

    let mut xp_for_next_level = 100;
    let mut current_total_xp = 0;
    while total_xp >= current_total_xp + xp_for_next_level {
        current_total_xp += xp_for_next_level;
        level += 1;
        xp_for_next_level = 5 * (level * level) + 50 * level + 100;
    }

    let xp = total_xp - current_total_xp;

    update_user_level_data(level_data.id, xp, total_xp, level).await?;
    update_member_roles(msg, ctx, level).await?;

    Ok(Some(()))
}

async fn update_member_roles(msg: &Message, ctx: &Context, level: i32) -> Result<Option<()>> {
    let member = msg.member(&ctx).await?;

    let highest_qualifying_role_id = LEVEL_ROLES
        .iter()
        .filter(|(role_level, _)| **role_level <= level)
        .max_by_key(|(role_level, _)| *role_level)
        .map(|(_, &id)| id);

    let highest_role_id = match highest_qualifying_role_id {
        Some(id) => id,
        None => return Ok(None),
    };

    member.add_role(&ctx, highest_role_id).await?;

    let roles_to_remove: Vec<&RoleId> = member
        .roles
        .iter()
        .filter(|&role_id| {
            *role_id != highest_role_id && LEVEL_ROLES.iter().any(|(_, &id)| id == *role_id)
        })
        .collect();

    for role in roles_to_remove {
        member.remove_role(&ctx, *role).await?
    }

    Ok(Some(()))
}
