import Discord from "discord.js"
import {ChannelType} from "discord-api-types/v10"

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("serverinfo")
        .setDescription("Get information about the server"),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return
        }

        const icon = interaction.guild.iconURL() || "";

        const embed = new Discord.EmbedBuilder()
            .setAuthor({name: interaction.guild.name, iconURL: icon})
            .addFields([
                {name: "Owner", value: `<@${interaction.guild.ownerId}>`, inline: true},
                {
                    name: "Channel Categories",
                    value: interaction.guild.channels.cache.filter(channel => channel.type === ChannelType.GuildCategory).size.toString(),
                    inline: true
                },
                {
                    name: "Text Channels",
                    value: interaction.guild.channels.cache.filter(channel => channel.type === ChannelType.GuildText).size.toString(),
                    inline: true
                },
                {
                    name: "Voice Channels",
                    value: interaction.guild.channels.cache.filter(channel => channel.type === ChannelType.GuildVoice).size.toString(),
                    inline: true
                },
                {name: "Members", value: interaction.guild.memberCount.toString(), inline: true},
                {name: "Roles", value: interaction.guild.roles.cache.size.toString(), inline: true}
            ])
            .setFooter({text: `ID: ${interaction.guild.id} | Server Created: ${interaction.guild.createdAt.getFullYear()}-${interaction.guild.createdAt.getMonth()}-${interaction.guild.createdAt.getDate()}`})
            .setThumbnail(icon)

        interaction.reply({embeds: [embed]}).then()
    },
}
