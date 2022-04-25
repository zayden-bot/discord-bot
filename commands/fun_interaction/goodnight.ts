import Discord from "discord.js";

module.exports = {
    commands: ["goodnight", "gn"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message: Discord.Message) => {
        const member = message.mentions.members?.first() || message.member
        if (!member) {
            return;
        }

        const imageConfig = require("../../configs/image_config.json")
        let arrayId = "global";
        if (message.author.id in imageConfig.goodNightImgs) {
            arrayId = message.author.id
        }

        const imgId = Math.floor(Math.random() * imageConfig.goodNightImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`Good Night, ${member.displayName}`)
            .setImage(imageConfig.goodNightImgs[arrayId][imgId])

        message.channel.send({embeds: [embed]})
    },
}
