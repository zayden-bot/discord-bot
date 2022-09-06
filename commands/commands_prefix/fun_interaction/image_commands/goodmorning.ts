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

        const embed = new Discord.EmbedBuilder()
            .setTitle(`Good Morning, ${member.displayName}`)
            .setImage(image)

        message.channel.send({embeds: [embed]})
    },
}
