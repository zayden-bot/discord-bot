import Discord from "discord.js";

module.exports = {
    commands: ["kiss"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message: Discord.Message) => {
        const member = message.mentions.members?.first() || message.member
        if (!member) {
            return;
        }

        const imageConfig = require("../../configs/image_config.json")
        let arrayId = "global";
        if (message.author.id in imageConfig.kissingImgs) {
            arrayId = message.author.id
        }

        const imgId = Math.floor(Math.random() * imageConfig.kissingImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`${message.author.username} kisses ${member.displayName}`)
            .setImage(imageConfig.kissingImgs[arrayId][imgId])
            .setColor("#FFC0CB")

        message.channel.send({embeds: [embed]})
    },
}
