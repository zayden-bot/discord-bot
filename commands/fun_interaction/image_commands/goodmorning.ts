import Discord from "discord.js";
import {ImageConfig} from "../../../models/images/imageConfigSchema";

module.exports = {
    commands: ["goodmorning", "gm"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: async (message: Discord.Message) => {
        const member = message.mentions.members?.first() || message.member
        if (!member) {
            return;
        }

        const goodMorningImages = await ImageConfig.findOne({category: "goodMorning"}).exec()
        let arrayId = "global";
        if (message.author.id in goodMorningImages.users) {
            arrayId = message.author.id
        }

        const imgId = Math.floor(Math.random() * goodMorningImages[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`Good Morning, ${member.displayName}`)
            .setImage(goodMorningImages[arrayId][imgId])

        message.channel.send({embeds: [embed]})
    },
}
