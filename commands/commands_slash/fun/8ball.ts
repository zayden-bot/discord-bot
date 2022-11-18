import Discord from "discord.js"

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("8ball")
        .setDescription("Ask the magic 8ball a question")
        .addStringOption(option =>
            option.setName("question")
                .setDescription("Ask your question")
                .setRequired(true)),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        const question = interaction.options.getString("question")

        const randomNumber = Math.floor(Math.random() * 3);
        const responses = require("../../../configs/8ball_responses.json");

        let randomIndex: number;
        switch (randomNumber) {
            case 0:
                randomIndex = Math.floor(Math.random() * responses.Yes.length)
                await interaction.reply(`> ${question}\n${responses.Yes[randomIndex]}`)
                break;
            case 1:
                randomIndex = Math.floor(Math.random() * responses.No.length)
                await interaction.reply(`> ${question}\n${responses.No[randomIndex]}`)
                break;
            case 2:
                randomIndex = Math.floor(Math.random() * responses.Maybe.length)
                await interaction.reply(`> ${question}\n${responses.Maybe[randomIndex]}`)
                break;
        }

    }
}
