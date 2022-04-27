import Discord from "discord.js";
import {getImage} from "./image_cmd_base";

module.exports = {
    commands: ["goodnight", "gn"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: async (message: Discord.Message) => {
        const member = message.mentions.members?.first() || message.member
        if (!member) {
            return;
        }

        const image = await getImage(message.author, "goodNight")

        const embed = new Discord.MessageEmbed()
            .setTitle(`Good Night, ${member.displayName}`)
            .setImage(image)

        message.channel.send({embeds: [embed]})
    },
}
