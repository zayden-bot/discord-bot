import Discord from "discord.js"
import {parseId} from "../../../common";
import {getConnection} from "../../../servers";
import {IReactionRole} from "../../../models/server_settings/ReactionRoleSchema";

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

        const channel = interaction.options.getChannel("channel", true) as Discord.TextChannel;
        const messageId = parseId(interaction.options.getString("message_id", true)) || "";
        const emoji = interaction.options.getString("emoji", true)

        // Handle Message
        const message = await channel.messages.fetch(messageId)

        if (!message) {
            return interaction.reply({content: "Invalid message.", ephemeral: true});
        }

        // Find ReactionRole
        const conn = getConnection(interaction.guild.id)
        const reactionRole = await conn.model<IReactionRole>("ReactionRoles").findOneAndDelete({
            channelId: channel.id,
            messageId: message.id,
            emoji: emoji
        })
        if (!reactionRole)
            return interaction.reply({content: "No reaction role found", ephemeral: true})

        const reaction = message.reactions.cache.get(emoji)
        if (!reaction) {
            return interaction.reply({content: "No reaction found", ephemeral: true});
        }

        await reaction.remove()
        await interaction.reply("Successfully removed reaction")
    }
}
