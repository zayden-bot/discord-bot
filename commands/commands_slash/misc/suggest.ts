import Discord from "discord.js"
import {getServer} from "../../../models/server";
import {ChannelType} from "discord-api-types/v10"

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("suggest")
        .setDescription("Make a suggestion and have the community vote")
        .addStringOption(option =>
            option.setName("suggestion")
                .setDescription("Your suggestion")
                .setRequired(true)),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const server = await getServer(interaction.guild.id)
        const text = interaction.options.getString("suggestion")

        const embed = new Discord.EmbedBuilder()
            .setTitle(`From: ${interaction.user.username}`)
            .setDescription(text);

        let channel = await interaction.guild.channels.fetch(server.channels.suggestionChannel);
        if (channel && channel.type == ChannelType.GuildText) {
            const message = await channel.send({embeds: [embed]})
            await message.react("👍");
            await message.react("👎");
        }

        interaction.reply({content: "Successfully created suggestion", ephemeral: true}).then()
    },
}
