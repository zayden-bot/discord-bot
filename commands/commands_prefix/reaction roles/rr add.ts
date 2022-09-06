import Discord from "discord.js"
import {IReactionRole, IServer} from "../../../models/server";
import {parseId} from "../../../common";
import {ChannelType} from "discord-api-types/v10"

module.exports = {
    commands: ["rr add", "rradd"],
    expectedArgs: "<channel> <message> <role> <emoji>",
    minArgs: 4,
    maxArgs: 4,
    callback: async (message: Discord.Message, server: IServer, args: string[]) => {
        const guild = message.guild;
        if (!guild) {
            return;
        }

        const channelId = parseId(args[0]);
        const messageId = parseId(args[1]);
        const roleId = parseId(args[2]);
        const emoji = args[3];

        if (!channelId || !messageId || !roleId) {
            await message.reply("Invalid arguments")
            return;
        }

        // Handle Channel
        const channel = await guild.channels.fetch(channelId);

        if (!channel || channel.type != ChannelType.GuildText) {
            await message.reply("Invalid channel.");
            return;
        }

        // Handle Message
        const msg: Discord.Message = await channel.messages.fetch(messageId)

        if (!msg) {
            await message.reply("Invalid message.");
            return;
        }

        // Handle Role
        const role: Discord.Role | null = await guild.roles.fetch(roleId);

        if (!role) {
            await message.reply("Invalid Role.");
            return;
        }

        // Create ReactionRole
        const reactionRole: IReactionRole = {
            channelId: channel.id,
            messageId: msg.id,
            roleId: role.id,
            emoji: emoji
        }
        server.reactionRoles.push(reactionRole);

        await Promise.all([
            msg.react(emoji),
            server.save(),
            message.reply("Successfully added reaction")
        ])
    },
    permissions: ["MANAGE_ROLES"]
}

// TODO: Check if role is allowed
