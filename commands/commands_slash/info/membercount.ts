import Discord from "discord.js"

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("membercount")
        .setDescription("View the total member count"),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        interaction.reply(`**${interaction.guild?.memberCount}** total members`).then()
    },
}
