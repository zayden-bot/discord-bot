import Discord from "discord.js"

module.exports = {
    commands: ["rrremove"],
    expectedArgs: "<channel> <message> <emoji>",
    minArgs: 3,
    maxArgs: 3,
    callback: async (message: Discord.Message, args: string[], text: string) => {
        const guild: Discord.Guild | null = message.guild;
        if (!guild) { return; }

        // Handle Channel
        const common = require("../../common")
        const channelId: string = common.parseId(args[0]);
        const channel: Discord.Channel | null = await guild.channels.fetch(channelId);

        if (!channel || !channel.isText()) { return message.reply("Invald channel."); }

        // Handle Message
        const messageId: string= common.parseId(args[1]);
        const msg: Discord.Message = await channel.messages.fetch(messageId)

        if (!msg) { return message.reply("Invalid message."); }

        // Handle Emoji
        const emojiId: string = common.parseId(args[2])
        const emoji: Discord.GuildEmoji = await guild.emojis.fetch(emojiId)

        // Create ReactionRole
        const status: boolean = require("./functions").removeReactionRole(guild, channel, msg, emoji)

        if (!status) { return message.reply("Failed to remove reaction") }

        message.reply("Successfully removed reaction")
    },
    permissions: ["MANAGE_ROLES"]
}