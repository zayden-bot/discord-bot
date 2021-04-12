const Discord = require("discord.js")
const botConfig = require("../../Configs/imgConfig.json")

module.exports = {
    commands: ["goodmorning", "gm"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let member = message.author.username
        if (text) { member = message.mentions.members.first().user.username }

        let arrayId = "Global"
        if (message.author.id in botConfig.goodMorningImgs) { arrayId = message.author.id }

        const imgId = Math.floor(Math.random() * botConfig.goodMorningImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`Good Morning, ${member}`)
            .setImage(botConfig.goodMorningImgs[arrayId][imgId])

        message.channel.send(embed)
    },
}
