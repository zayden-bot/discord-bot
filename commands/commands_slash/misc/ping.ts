import Discord from "discord.js";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("ping")
        .setDescription("Replies with Pong!"),
    async execute(interaction: Discord.ChatInputCommandInteraction) {
        await interaction.reply("Pong!")
    }
}
