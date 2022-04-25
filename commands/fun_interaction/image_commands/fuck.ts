import Discord from "discord.js"

module.exports = {
    commands: ["fuck"],
    expectedArgs: "<user>",
    maxArgs: 1,
    cooldown: 300,
    callback: (message: Discord.Message) => {
        if (message.channel.id != "831959023662137394") {
            return;
        }

        const member = message.mentions.members?.first() || message.member
        if (!member) {
            return;
        }

        const imageConfig = require("../../../configs/image_config.json")
        let arrayId = "global";
        if (message.author.id in imageConfig.fuckingImgs) {
            arrayId = message.author.id
        }

        const imgId = Math.floor(Math.random() * imageConfig.fuckingImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`${message.author.username} fucks ${member.displayName}`)
            .setImage(imageConfig.fuckingImgs[arrayId][imgId])

        message.channel.send({embeds: [embed]})
    }
}
