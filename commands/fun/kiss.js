const Discord = require("discord.js")
const botConfig = require("../../botConfig.json")

module.exports = {
    commands: ["kiss"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let member = message.author.username
        if (text) { member = message.mentions.members.first().user.username }

        let arrayId = "Global"
        if (message.author.id in botConfig.kissingImgs) { arrayId = message.author.id }

        const imgId = Math.floor(Math.random() * botConfig.kissingImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`${message.author.username} kisses ${member}`)
            .setImage(botConfig.kissingImgs[arrayId][imgId])
            .setColor("FFC0CB")

        message.channel.send(embed)
    },
}
