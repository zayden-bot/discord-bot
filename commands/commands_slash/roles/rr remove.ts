import Discord from "discord.js"
import {parseId} from "../../../common";
import {getServer} from "../../../models/server";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("reactionrole_remove")
        .setDescription("Remove a reaction role")
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
        const emoji = interaction.options.getString("emoji", true)

        // Handle Message
        const message = await channel.messages.fetch(messageId)

        if (!message) {
            return interaction.reply({content: "Invalid message.", ephemeral: true});
        }

        // Find ReactionRole
        const reactionRoleIndex = server.reactionRoles.findIndex((element) => {
            return element.channelId == channel.id && element.messageId == message.id && element.emoji == emoji
        })

        if (reactionRoleIndex == -1) {
            return interaction.reply({content: "No reaction role found", ephemeral: true});
        }
        server.reactionRoles.splice(reactionRoleIndex, 1);

        const reaction = message.reactions.cache.get(emoji)
        if (!reaction) {
            return interaction.reply({content: "No reaction found", ephemeral: true});
        }

        await reaction.remove()

        await Promise.all([
            await server.save(),
            await interaction.reply("Successfully removed reaction")
        ])
    }
}
