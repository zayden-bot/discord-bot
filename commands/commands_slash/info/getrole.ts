import Discord from "discord.js"

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("getrole")
        .setDescription("How do I get my Discord role"),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        const embed = new Discord.EmbedBuilder()
            .setTitle("How do I get my Discord role when I become a patron?")
            .setURL(`https://support.patreon.com/hc/en-us/articles/212052266-Get-my-Discord-role`)
            .addFields([
                {
                    name: "Step 1",
                    value: "Make sure you’re in the right Tier. If you made a “custom pledge,” instead of joining a Tier, you’ll not be assigned any Discord roles."
                },
                {
                    name: "Step 2",
                    value: "After you confirm your payment amount, and Tier selection, you’ll be taken to your creator’s Welcome note. You can get started by clicking the **Connect to Discord** button."
                },
                {
                    name: "Step 3",
                    value: "You'll be taken to the App section of your __Profile settings__ page – click the **Connect** button to the right of the Discord app. Log in to your Discord account in the pop-up window that populates.\""
                },
                {
                    name: "Step 4",
                    value: "Now that your Patreon and Discord accounts are communicating, our integration will assign you the role tied to your Tier!"
                }
            ])
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
            .setFooter({text: "https://www.patreon.com/collegekings"})
            .setColor("#ff0000")

        interaction.reply({embeds: [embed]}).then()
    },
}
