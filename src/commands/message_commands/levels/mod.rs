pub mod user_levels;

use chrono::{TimeDelta, Utc};
use lazy_static::lazy_static;
use rand::Rng;
use serenity::all::{Context, Member, Message};
use std::collections::HashMap;

use user_levels::{get_user_level_data, update_user_level_data};

const BLOCKED_CHANNEL_IDS: [u64; 1] = [776139754408247326];

lazy_static! {
    static ref LEVEL_ROLES: HashMap<i32, u64> = {
        let mut map = HashMap::new();
        map.insert(5, 787443819024220210); // New Fan | Level 5
        map.insert(10, 787445571539304510); // Active Fan | Level 10
        map.insert(20, 787445900992577556); // Big Fan | Level 20
        map.insert(40, 787446715057831976); // Super Fan | Level 40
        map.insert(60, 787447090728796191); // Mega Fan | Level 60
        map.insert(80, 787447252783202326); // Ultra Fan | Level 80
        map
    };
}

pub async fn run(ctx: &Context, msg: &Message) {
    if msg.guild_id.is_none() {
        return;
    }

    let channel_id = msg.channel_id;

    if BLOCKED_CHANNEL_IDS.contains(&channel_id.get()) {
        return;
    }

    let level_data = match get_user_level_data(msg.author.id.get()).await {
        Ok(data) => data,
        Err(why) => {
            println!("Cannot get user level data: {}", why);
            return;
        }
    };

    if level_data.last_xp >= (Utc::now().naive_utc() - TimeDelta::try_minutes(1).unwrap()) {
        return;
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

    if let Err(why) = update_user_level_data(level_data.id, xp, total_xp, level).await {
        println!("Cannot update user level data: {}", why);
    }

    if let Some(role_id) = LEVEL_ROLES.get(&level) {
        let member = msg.member(&ctx).await.unwrap();

        if let Err(why) = member.add_role(&ctx, *role_id).await {
            println!("Cannot add role: {}", why);
        }

        for (role_level, role_id) in LEVEL_ROLES.iter() {
            if *role_level < level {
                if let Err(why) = member.remove_role(&ctx, *role_id).await {
                    println!("Cannot remove role: {}", why);
                }
            }
        }
    }
}
