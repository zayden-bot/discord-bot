import Discord from "discord.js"
import {parseId} from "../../../common";
import {getConnection} from "../../../servers";
import {IReactionRole} from "../../../models/server_settings/ReactionRoleSchema";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("reactionrole_add")
        .setDescription("Create a reaction role")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.ManageMessages)
        .addChannelOption(option =>
            option.setName("channel")
                .setDescription("Target channel for reaction message")
                .setRequired(true)
                .addChannelTypes(Discord.ChannelType.GuildText))
        .addStringOption(option =>
            option.setName("message_id")
                .setDescription("Enter reaction message ID")
                .setRequired(true))
        .addRoleOption(option =>
            option.setName("role")
                .setDescription("Set reaction role")
                .setRequired(true))
        .addStringOption(option =>
            option.setName("emoji")
                .setDescription("Enter emoji for reaction")
                .setRequired(true)),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const conn = getConnection(interaction.guild.id)
        const reactionRoles = await conn.model<IReactionRole>("ReactionRoles")

        const channel = interaction.options.getChannel("channel", true) as Discord.TextChannel;
        const messageId = parseId(interaction.options.getString("message_id", true)) || "";
        const role = interaction.options.getRole("role", true)
        const emoji = interaction.options.getString("emoji", true)

        // Handle Message
        const message = await channel.messages.fetch(messageId)

        if (!message) {
            return interaction.reply({content: "Invalid message.", ephemeral: true});
        }

        // Create ReactionRole
        await (await reactionRoles.create({
            channelId: channel.id,
            messageId: message.id,
            roleId: role.id,
            emoji: emoji
        })).save()

        await Promise.all([
            message.react(emoji),
            interaction.reply("Successfully added reaction")
        ])
    },
}
