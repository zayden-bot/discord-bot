const Discord = require("discord.js")
const botConfig = require("../../Configs/imgConfig.json")

module.exports = {
    commands: ["hug"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let member = message.author.username
        if (text) { member = message.mentions.members.first().user.username }

        let arrayId = "Global"
        if (message.author.id in botConfig.huggingImgs) { arrayId = message.author.id }

        const imgId = Math.floor(Math.random() * botConfig.huggingImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`Sending hugs to ${member}`)
            .setImage(botConfig.huggingImgs[arrayId][imgId])
            .setColor("FFC0CB")

        message.channel.send(embed)
    },
}
