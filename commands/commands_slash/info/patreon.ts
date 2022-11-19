import Discord from "discord.js"

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("patreon")
        .setDescription("Get patreon link and info"),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        const embed = new Discord.EmbedBuilder()
            .setTitle("Pledge to College Kings")
            .setURL(`https://www.patreon.com/collegekings`)
            .setDescription("**Interested In Getting Early Updates, Patron-only behind the scenes/post... and more?\n\nCheck it all out here!**\nhttps://www.patreon.com/collegekings")
            .setImage("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg?width=1440&height=360")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
            .setFooter({text: "https://www.patreon.com/collegekings"})

        interaction.reply({embeds: [embed]}).then()
    },
}
