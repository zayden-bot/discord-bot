const Discord = require("discord.js")
const config = require("../../serverConfigs/CKConfig.json")

module.exports = {
    commands: ["kiss"],
    expectedArgs: "<user>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const imgId = Math.floor(Math.random() * config.kissingImgs.length)
        const member = message.mentions.members.first()
        const embed = new Discord.MessageEmbed()
            .setTitle(`${message.author.username} kisses ${member.user.username}`)
            .setImage(config.kissingImgs[imgId])
            .setColor("FFC0CB")
        // console.log(kissingImages[imgId])
        message.channel.send(embed)
    },
}