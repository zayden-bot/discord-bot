import Discord from "discord.js"

export function addNormalReaction(guild: Discord.Guild, channel: Discord.TextChannel, message: Discord.Message, role: Discord.Role, emoji: string): boolean {
    const { ReactionRole } = require("../../reactionRole");
    const { Server, servers } = require("../../server");

    // Get server object
    let server = servers[guild.id];
    if (!server) { server = new Server(guild.id) }

    // Create reaction role and push to server
    const reactionRole = new ReactionRole(channel, message, role, emoji)
    server.reactionRoles.push(reactionRole);
    
    // Add to JSON
    const fs = require("fs")
    fs.writeFileSync(`./server_configs/${guild.id}.json`, JSON.stringify(server, null, 4), (error: any) => {
        if (error) { return console.log(error); }
    });

    message.react(emoji)
    return true;
}