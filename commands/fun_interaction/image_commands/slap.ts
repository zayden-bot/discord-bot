import Discord from "discord.js";

module.exports = {
    commands: ["slap"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message: Discord.Message) => {
        const member = message.mentions.members?.first() || message.member
        if (!member) {
            return;
        }

        const imageConfig = require("../../../configs/image_config.json")
        let arrayId = "global";
        if (message.author.id in imageConfig.slapImages) {
            arrayId = message.author.id
        }

        const imgId = Math.floor(Math.random() * imageConfig.slapImages[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`${message.author.username} slaps ${member.displayName}`)
            .setImage(imageConfig.slapImages[arrayId][imgId])

        message.channel.send({embeds: [embed]})
    },
}
