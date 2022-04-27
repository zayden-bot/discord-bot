import Discord from "discord.js"
import {parseId} from "../../common";
import {IServer} from "../../models/server";

module.exports = {
    commands: ["rrremove"],
    expectedArgs: "<channel> <message> <emoji>",
    minArgs: 3,
    maxArgs: 3,
    callback: async (message: Discord.Message, server: IServer, args: string[]) => {
        const guild = message.guild;
        if (!guild) {
            return;
        }

        const channelId = parseId(args[0]);
        const messageId = parseId(args[1]);
        const emoji = args[2];

        if (!channelId || !messageId) {
            await message.reply("Invalid arguments")
            return;
        }

        // Handle Channel
        const channel = await guild.channels.fetch(channelId);

        if (!channel || !channel.isText()) {
            return message.reply("Invalid channel.");
        }

        // Handle Message
        const msg: Discord.Message = await channel.messages.fetch(messageId)

        if (!msg) {
            return message.reply("Invalid message.");
        }

        // Create ReactionRole
        const reactionRoleIndex = server.reactionRoles.findIndex((element) => {
            return element.channelId == channel.id && element.messageId == msg.id && element.emoji == emoji
        })

        if (reactionRoleIndex == -1) {
            console.log("No reaction role found")
            return false;
        }
        server.reactionRoles.splice(reactionRoleIndex, 1);

        const reaction = msg.reactions.cache.get(emoji)
        if (!reaction) {
            console.log("No reaction found")
            return false;
        }

        await reaction.remove()

        await Promise.all([
            await server.save(),
            await message.reply("Successfully removed reaction")
        ])
    },
    permissions: ["MANAGE_ROLES"]
}