const Discord = require("discord.js")
const config = require("../../../serverConfigs/CKConfig.json")

module.exports = {
    commands: ["goodnight", "gn"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let member = message.author.username
        if (text) { member = message.mentions.members.first().user.username }

        let arrayId = "Global"
        if (message.author.id in config.goodNightImgs) { arrayId = message.author.id }

        const imgId = Math.floor(Math.random() * config.goodNightImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`Good Night, ${member}`)
            .setImage(config.goodNightImgs[arrayId][imgId])

        message.channel.send(embed)
    },
}