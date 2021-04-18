const Discord = require("discord.js")
const imgConfig = require("../../configs/imgConfig.json")

module.exports = {
    commands: ["hug"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let member;
        try { member = message.mentions.members.first().user.username }
        catch (error) { member = message.author.username }

        let arrayId = "Global";
        if (message.author.id in imgConfig.huggingImgs) { arrayId = message.author.id }
        else {
            try {
                if (message.mentions.members.first().user.id in imgConfig.huggingImgs) { arrayId = message.mentions.members.first().user.id }
            }
            catch (error) { arrayId = "Global" }
        }

        const imgId = Math.floor(Math.random() * imgConfig.huggingImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`Sending hugs to ${member}`)
            .setImage(imgConfig.huggingImgs[arrayId][imgId])
            .setColor("FFC0CB")

        message.channel.send(embed)
    },
}
