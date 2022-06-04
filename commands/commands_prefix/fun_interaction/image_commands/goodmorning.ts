import Discord from "discord.js";
import {getImage} from "./image_cmd_base";

module.exports = {
    commands: ["goodmorning", "gm"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: async (message: Discord.Message) => {
        const member = message.mentions.members?.first() || message.member
        if (!member) {
            return;
        }

        const image = await getImage(message.author, "goodMorning")
        if (!image) {
            return message.reply("No \"good morning\" image found")
        }
        
        const embed = new Discord.MessageEmbed()
            .setTitle(`Good Morning, ${member.displayName}`)
            .setImage(image)

        message.channel.send({embeds: [embed]})
    },
}
