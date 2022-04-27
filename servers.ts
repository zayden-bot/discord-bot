import {Server} from "./models/servers/server";
import Discord from "discord.js";

export let servers: Record<string, any> = {}

export function createServer(guild: Discord.Guild,) {
    const server = new Server({id: guild.id})
    servers[guild.id] = server
    server.save()
}

export async function init(client: Discord.Client) {
    for (const g of await client.guilds.fetch()) {
        const guild = await client.guilds.fetch(g[0])

        await createServer(guild)

        // Cache reaction messages
        for (let reactionRole of servers[guild.id].reactionRoles) {
            const channel = await client.channels.fetch(reactionRole.channelId)
            if (!channel || channel.type != "GUILD_TEXT") {
                break;
            }

            const msg = await channel.messages.fetch(reactionRole.messageId)
            reactionRole.messageId = msg.id

            const role = await guild.roles.fetch(reactionRole.roleId)
            reactionRole.roleId = role?.id
        }
    }
}
