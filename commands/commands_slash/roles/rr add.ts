import Discord from "discord.js"
import {getServer, IReactionRole} from "../../../models/server";
import {parseId} from "../../../common";

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

        const server = await getServer(interaction.guild.id)

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
        const reactionRole: IReactionRole = {
            channelId: channel.id,
            messageId: message.id,
            roleId: role.id,
            emoji: emoji
        }
        server.reactionRoles.push(reactionRole);

        await Promise.all([
            message.react(emoji),
            server.save(),
            interaction.reply("Successfully added reaction")
        ])
    },
}
