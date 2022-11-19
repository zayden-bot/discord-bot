import Discord from "discord.js";
import {ChannelType} from "discord-api-types/v10"

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("purgeall")
        .setDescription("Bulk delete messages in a channel")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.Administrator),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.channel) {
            return
        }

        const results = await interaction.channel.messages.fetch()
        if (interaction.channel.type != ChannelType.DM) {
            interaction.channel.bulkDelete(results)
        }

        interaction.reply({content: "Bulk delete successful", ephemeral: true}).then()
    },
}
