const Discord = require("discord.js")
const imgConfig = require("../../configs/imgConfig.json")

module.exports = {
    commands: ["wisdomoftheday", "wisdom", "w"],
    callback: (message, arguments, text) => {

        const imgId = Math.floor((new Date() - new Date(now.getFullYear(), 0, 0)) / 86400000);

        const embed = new Discord.MessageEmbed()
            .setImage(imgConfig.goodMorningImgs[imgId])

        message.channel.send(embed)
    },
}
// TODO: Add try/catch for the image
