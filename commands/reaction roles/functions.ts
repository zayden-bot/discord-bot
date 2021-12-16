import Discord from "discord.js"
import { ReactionRole } from "./reactionRole";

function getReactionRoleIndex(reactionRoles: ReactionRole[], channel: Discord.Channel, message: Discord.Message, emoji: string) {
    for (let i = 0; i < reactionRoles.length; i++) {
        const reactionRole = reactionRoles[i]
        if (reactionRole.channelId === channel.id && reactionRole.messageId === message.id, reactionRole.emoji === emoji) { return i; }
    }

    return undefined;
}

export function addNormalReaction(guild: Discord.Guild, channel: Discord.TextChannel, message: Discord.Message, role: Discord.Role, emoji: string): boolean {
    const { ReactionRole } = require("./reactionRole");
    const { Server, servers } = require("../../server");

    // Get server object
    let server = servers[guild.id];
    if (!server) { server = new Server(guild.id) }

    // Create reaction role and push to server
    const reactionRole = new ReactionRole(channel.id, message.id, role.id, emoji)
    server.reactionRoles.push(reactionRole);
    
    // Add to JSON
    const fs = require("fs")
    fs.writeFileSync(`./server_configs/${guild.id}.json`, JSON.stringify(server, null, 4), (error: any) => {
        if (error) { return console.log(error); }
    });

    message.react(emoji)
    .catch((error: any) => {
        console.log(error);
        return false;
    })
    return true;
}

export function removeReactionRole(guild: Discord.Guild, channel: Discord.TextChannel, message: Discord.Message, emoji: string): boolean {
    const { Server, servers } = require("../../server");

    // Get server object
    let server = servers[guild.id];
    if (!server) { server = new Server(guild.id) }

    // Remove reaction role from server
    const reactionRoleIndex = getReactionRoleIndex(server.reactionRoles, channel, message, emoji)
    if (typeof(reactionRoleIndex) === "undefined") {
        console.log("No reaction role found")
        return false;
    }
    server.reactionRoles.splice(reactionRoleIndex, 1);
    
    // Add to JSON
    const fs = require("fs")
    fs.writeFileSync(`./server_configs/${guild.id}.json`, JSON.stringify(server, null, 4), (error: any) => {
        if (error) { return console.log(error); }
    });

    const reaction = message.reactions.cache.get(emoji)
    if (!reaction) {
        console.log("No reaction found")
        return false;
    }

    reaction.remove()

    return true;
}