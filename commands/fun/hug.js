const Discord = require("discord.js")
const config = require("../../serverConfigs/CKConfig.json")

module.exports = {
    commands: ["hug"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let member = message.author.username
        if (text) { member = message.mentions.members.first().user.username }

        let arrayId = "Global"
        if (message.author.id in config.huggingImgs) { arrayId = message.author.id }

        const imgId = Math.floor(Math.random() * config.huggingImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`${message.author.username} hugs ${member.user.username}`)
            .setImage(config.huggingImgs[arrayId][imgId])
            .setColor("FFC0CB")

        message.channel.send(embed)
    },
}
