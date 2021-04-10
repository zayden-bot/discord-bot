const Discord = require("discord.js")
const config = require("../../serverConfigs/CKConfig.json")

module.exports = {
    commands: ["kiss"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let member = message.author.username
        if (text) { member = message.mentions.members.first().user.username }

        let arrayId = "Global"
        if (message.author.id in config.kissingImgs) { arrayId = message.author.id }

        const imgId = Math.floor(Math.random() * config.kissingImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`${message.author.username} kisses ${member.user.username}`)
            .setImage(config.kissingImgs[arrayId][imgId])
            .setColor("FFC0CB")

        message.channel.send(embed)
    },
}
