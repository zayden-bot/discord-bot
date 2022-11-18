import Discord from "discord.js";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("wisdom")
        .setDescription("View the daily wisdom message"),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        // Returns 0 - 365
        const now = new Date();
        const start = new Date(now.getFullYear(), 0, 0);
        const oneDay = 1000 * 60 * 60 * 24;
        const imageIndex = Math.floor((now.valueOf() - start.valueOf()) / oneDay)

        const embed = new Discord.EmbedBuilder()
            .setTitle(`Daily Wisdom #${imageIndex}`)
            .setImage(`https://condycandy.com/wp-content/uploads/2022/11/${imageIndex}.jpg`)

        interaction.reply({embeds: [embed]}).then()
    }
}
