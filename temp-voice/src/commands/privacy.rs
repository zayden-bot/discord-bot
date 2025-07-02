use std::collections::HashMap;

use serenity::all::{
    ChannelId, CommandInteraction, Context, EditChannel, EditInteractionResponse, GuildId,
    PermissionOverwrite, PermissionOverwriteType, Permissions, ResolvedValue, RoleId, UserId,
};

use crate::error::PermissionError;
use crate::{Error, VoiceChannelRow, VoiceStateCache};

pub async fn privacy(
    ctx: &Context,
    interaction: &CommandInteraction,
    mut options: HashMap<&str, ResolvedValue<'_>>,
    guild_id: GuildId,
    channel_id: ChannelId,
    row: VoiceChannelRow,
) -> Result<(), Error> {
    interaction.defer_ephemeral(ctx).await.unwrap();

    if !row.is_trusted(interaction.user.id) {
        return Err(Error::MissingPermissions(PermissionError::NotTrusted));
    }

    let privacy = match options.remove("privacy") {
        Some(ResolvedValue::String(privacy)) => privacy,
        _ => "visible",
    };

    let everyone_role = guild_id.everyone_role();

    let channel = channel_id.to_channel(ctx).await.unwrap().guild().unwrap();
    let users = {
        let data = ctx.data.read().await;
        let cache = data.get::<VoiceStateCache>().unwrap();
        cache
            .values()
            .filter(|state| state.channel_id == Some(channel_id))
            .map(|state| state.user_id)
            .collect::<Vec<_>>()
    };

    let perms = channel.permission_overwrites;

    let builder = match privacy {
        "open" => open_builder(perms, everyone_role),
        "spectator" => spectate_builder(perms, everyone_role, users),
        "lock" => lock_builder(perms, everyone_role),
        "invisible" => invisible_builder(perms, everyone_role),
        _ => unreachable!("Invalid privacy option"),
    };

    channel_id.edit(ctx, builder).await.unwrap();

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Channel privacy updated."),
        )
        .await
        .unwrap();

    Ok(())
}

fn open_builder<'a>(perms: Vec<PermissionOverwrite>, everyone: RoleId) -> EditChannel<'a> {
    let perms = perms.into_iter().map(|perm| {
        if perm.kind == PermissionOverwriteType::Role(everyone) {
            PermissionOverwrite {
                allow: Permissions::VIEW_CHANNEL,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Role(everyone),
            }
        } else {
            perm
        }
    });

    EditChannel::new().permissions(perms)
}

fn spectate_builder<'a>(
    mut perms: Vec<PermissionOverwrite>,
    everyone: RoleId,
    users: Vec<UserId>,
) -> EditChannel<'a> {
    for user in users {
        let perm = perms.iter_mut().find(
            |perm| matches!(perm.kind, PermissionOverwriteType::Member(user_id) if user_id == user),
        );

        if let Some(perm) = perm {
            perm.allow |= Permissions::SPEAK;
        } else {
            perms.push(PermissionOverwrite {
                allow: Permissions::SPEAK,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(user),
            });
        }
    }

    match perms
        .iter_mut()
        .find(|perm| matches!(perm.kind, PermissionOverwriteType::Role(role) if role == everyone))
    {
        Some(perm) => perm.deny |= Permissions::SPEAK,
        None => perms.push(PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::SPEAK,
            kind: PermissionOverwriteType::Role(everyone),
        }),
    }

    EditChannel::new().permissions(perms)
}

fn lock_builder<'a>(perms: Vec<PermissionOverwrite>, everyone: RoleId) -> EditChannel<'a> {
    let perms = perms.into_iter().map(|perm| {
        if perm.kind == PermissionOverwriteType::Role(everyone) {
            PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::CONNECT,
                kind: PermissionOverwriteType::Role(everyone),
            }
        } else {
            perm
        }
    });

    EditChannel::new().permissions(perms)
}

fn invisible_builder<'a>(perms: Vec<PermissionOverwrite>, everyone: RoleId) -> EditChannel<'a> {
    let perms = perms.into_iter().map(|perm| {
        if perm.kind == PermissionOverwriteType::Role(everyone) {
            PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::VIEW_CHANNEL,
                kind: PermissionOverwriteType::Role(everyone),
            }
        } else {
            perm
        }
    });

    EditChannel::new().permissions(perms)
}
