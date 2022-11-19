import Discord from "discord.js"

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("reputation")
        .setDescription("View the secrets behind the reputation value"),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        const embed = new Discord.EmbedBuilder()
            .addFields([
                {name: "Popular", value: `✅ Bro\n✅ Trouble Maker\n❌ Boyfriend`, inline: true},
                {name: "Loyal", value: `✅ Bro\n✅ Boyfriend\n❌ Trouble Maker`, inline: true},
                {name: "Confident", value: `✅ Boyfriend\n✅ Trouble Maker\n❌ Bro`, inline: true},
            ])
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")

        interaction.reply({embeds: [embed]}).then()
    },
}
