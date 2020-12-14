module.exports = {
    commands: ["update"],
    minArgs: 0,
    maxArgs: 0,
    callback: (message, arguments, text) => {
        const embed = new Discord.MessageEmbed()
            .setTitle("Next College Kings Update")
            .setColor("ff0000")
            .setDescription("**If you are interested in the next update, read below:**")
            .addField("Patreon Release", "**January 1st**\n***Junior Tier and up***", true)
            .addField("Public/Steam Release", "**January 16th**", true)
            .setImage("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg?width=1440&height=360")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
            .setFooter("https://www.patreon.com/collegekings")

        message.reply(embed)
    },
}