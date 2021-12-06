const Discord = require("discord.js")
// import { ReactionRole } from "../../reactionRole";
// import { Server, servers } from "../../server"


module.exports = {
    commands: ["rr add", "rradd"],
    expectedArgs: "<channel> <message> <role> <emoji>",
    minArgs: 4,
    maxArgs: 4,
    callback: (message, args, text) => {
        const guild = message.guild;
        if (!guild) return;

        // Handle Channel
        const common = require("../../common")
        const channelId = common.getChannelId(args[0]);
        let rrChannel = message.guild.channels.cache.get(channelId)
        if (!rrChannel) return;

        if (!rrChannel) {
            message.reply("Invald channel.");
            return;
        }

        // Handle Role
        const roleId = common.getRoleId(args[2]);
        const rrRole = guild.roles.cache.get(roleId);
        if (!rrRole) {
            message.reply("Invald Role.");
            return;
        }

        // Handle Emoji
        const rrEmoji = args[3]

        // Handle Message
        rrChannel.messages.fetch(args[1])
        .then(rrMessage => {
            if (!rrMessage) { 
                message.reply("Invald message.");
                return;
            }

            // Append reaction to server object
            const { ReactionRole } = require("../../reactionRole");
            const { Server, servers } = require("../../server");
            let server = servers[guild.id];
            if (!server) { server = new Server(guild.id) }

            const reactionRole = new ReactionRole(rrChannel, rrMessage, rrRole, rrEmoji)
            server.reactionRoles.push(reactionRole);

            rrMessage.react(rrEmoji)


            // Add to JSON
            const fs = require("fs")
            fs.writeFileSync(`./server_configs/${guild.id}.json`, JSON.stringify(server, null, 4), function writeJSON(err) {
                if (err) { return console.log(err); }
            });
        })
        .catch(console.error)



        message.reply("Successfully added reaction")
    },
    permissions: ["ADMINISTRATOR"],
}