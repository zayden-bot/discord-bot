import Discord from "discord.js";
import {getImage} from "./image_cmd_base";

module.exports = {
    commands: ["slap"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: async (message: Discord.Message) => {
        const member = message.mentions.members?.first() || message.member
        if (!member) {
            return;
        }

        const image = await getImage(message.author, "slap")
        if (!image) {
            return message.reply("No \"slap\" image found")
        }

        const embed = new Discord.MessageEmbed()
            .setTitle(`${message.author.username} slaps ${member.displayName}`)
            .setImage(image);

        message.channel.send({embeds: [embed]})
    },
}
