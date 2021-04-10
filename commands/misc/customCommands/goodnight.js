const Discord = require("discord.js")
const botConfig = require("../../../botConfig.json")

module.exports = {
    commands: ["goodnight", "gn"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let member = message.author.username
        if (text) { member = message.mentions.members.first().user.username }

        let arrayId = "Global"
        if (message.author.id in botConfig.goodNightImgs) { arrayId = message.author.id }

        const imgId = Math.floor(Math.random() * botConfig.goodNightImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`Good Night, ${member}`)
            .setImage(botConfig.goodNightImgs[arrayId][imgId])

        message.channel.send(embed)
    },
}