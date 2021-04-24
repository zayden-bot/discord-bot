const Discord = require("discord.js")
const imgConfig = require("../../configs/imgConfig.json")

module.exports = {
    commands: ["goodmorning", "gm"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let member;
        try { member = message.mentions.members.first().user.username }
        catch (error) { member = message.author.username }

        let arrayId = "Global";
        if (message.author.id in imgConfig.goodMorningImgs) { arrayId = message.author.id }
        else {
            try {
                if (message.mentions.members.first().user.id in imgConfig.fuckingImgs) { arrayId = message.mentions.members.first().user.id }
            } catch {}
        }

        const imgId = Math.floor(Math.random() * imgConfig.goodMorningImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`Good Morning, ${member}`)
            .setImage(imgConfig.goodMorningImgs[arrayId][imgId])

        message.channel.send(embed)
    },
}
