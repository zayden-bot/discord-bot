import Discord from "discord.js"

module.exports = {
    commands: ["rr add", "rradd"],
    expectedArgs: "<channel> <message> <role> <emoji>",
    minArgs: 4,
    maxArgs: 4,
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

        // Handle Role
        const roleId: string = common.parseId(args[2]);
        const role: Discord.Role | null = await guild.roles.fetch(roleId);

        if (!role) { return message.reply("Invald Role."); }

        // Handle Emoji
        const emoji = args[3];

        // Create ReactionRole
        const status: boolean = require("./functions").addNormalReaction(guild, channel, msg, role, emoji)

        if (!status) { return message.reply("Failed to added reaction") }

        message.reply("Successfully added reaction")
    },
    permissions: ["MANAGE_ROLES"]
}

// TODO: Check if role is allowed