const Discord = require("discord.js")
const imgConfig = require("../../configs/image_config.json")

module.exports = {
    commands: ["goodnight", "gn"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let member;
        try { member = message.mentions.members.first().user.username }
        catch (error) { member = message.author.username }

        let arrayId = "Global";
        if (message.author.id in imgConfig.goodNightImgs) { arrayId = message.author.id }
        else {
            try {
                if (message.mentions.members.first().user.id in imgConfig.fuckingImgs) { arrayId = message.mentions.members.first().user.id }
            } catch {}
        }

        const imgId = Math.floor(Math.random() * imgConfig.goodNightImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`Good Night, ${member}`)
            .setImage(imgConfig.goodNightImgs[arrayId][imgId])

        message.channel.send({embeds: [embed]})
    },
}
