import Discord from "discord.js";
import {ChannelType} from 'discord-api-types/v10';
import {IReactionRole} from "./models/server_settings/ReactionRoleSchema";
import {connectionFactory} from "./mongoDb";
import mongoose from "mongoose";

async function cacheReactionMessages(client: Discord.Client, guildId: string) {
    const guild = await client.guilds.fetch(guildId)
    const mongoDb = connectionFactory(guild.id)

    let reactionRoles: IReactionRole[] = await mongoDb.model("ReactionRoles").find();

    // Cache reaction messages
    for (let reactionRole of reactionRoles) {
        const channel = await client.channels.fetch(reactionRole.channelId)
        if (!channel || channel.type != ChannelType.GuildText) {
            break;
        }

        const msg = await channel.messages.fetch(reactionRole.messageId)
        reactionRole.messageId = msg.id

        const role = await guild.roles.fetch(reactionRole.roleId)
        reactionRole.roleId = role!.id
    }
}

export async function init(client: Discord.Client) {
    const tasks = []
    for (const g of await client.guilds.fetch()) {
        tasks.push(cacheReactionMessages(client, g[0]))
    }

    await Promise.all(tasks)
}

const connections: Record<string, mongoose.Connection> = {}

export function getConnection(connectionId: string) {
    if (connections.hasOwnProperty(connectionId)) {
        return connections[connectionId]
    }

    connections[connectionId] = connectionFactory(connectionId)
    return connections[connectionId]
}
