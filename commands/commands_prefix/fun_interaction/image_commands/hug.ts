import Discord from "discord.js";
import {getImage} from "./image_cmd_base";


module.exports = {
    commands: ["hug"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: async (message: Discord.Message) => {
        const member = message.mentions.members?.first() || message.member
        if (!member) {
            return;
        }

        const image = await getImage(message.author, "hug")
        if (!image) {
            return message.reply("No \"hug\" image found")
        }

        const embed = new Discord.EmbedBuilder()
            .setTitle(`Sending hugs to ${member.displayName}`)
            .setImage(image)
            .setColor("#FFC0CB")

        message.channel.send({embeds: [embed]})
    },
}
