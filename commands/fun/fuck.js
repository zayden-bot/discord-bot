const Discord = require("discord.js")
const botConfig = require("../../botConfig.json")

module.exports = {
    commands: ["fuck"],
    expectedArgs: "<user>",
    maxArgs: 1,
    cooldown: 43200,
    callback: (message, arguments, text) => {
        if (!message.channel.nsfw) {
            message.reply("This command can only be used in <#747428952577933424>")
            return
        }

        let member = message.author.username
        if (text) { member = message.mentions.members.first().user.username }

        let arrayId = "Global"
        if (message.author.id in botConfig.fuckingImgs) { arrayId = message.author.id }

        const imgId = Math.floor(Math.random() * botConfig.fuckingImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`${message.author.username} fucks ${member}`)
            .setImage(botConfig.fuckingImgs[arrayId][imgId])
            .setColor("FFC0CB")

        message.channel.send(embed)
    },
}
