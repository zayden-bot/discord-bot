const Discord = require("discord.js")
const config = require("../../../serverConfigs/CKConfig.json")

module.exports = {
    commands: ["goodmorning", "gm"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let member = message.author.username
        if (text) { member = message.mentions.members.first().user.username }

        let arrayId = "Global"
        if (message.author.id in config.goodMorningImgs) { arrayId = message.author.id }

        const imgId = Math.floor(Math.random() * config.goodMorningImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`Good Morning, ${member}`)
            .setImage(config.goodMorningImgs[arrayId][imgId])

        message.channel.send(embed)
    },
}