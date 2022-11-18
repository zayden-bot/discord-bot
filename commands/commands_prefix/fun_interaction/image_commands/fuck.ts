import Discord from "discord.js"
import {getImage} from "./image_cmd_base";

module.exports = {
    commands: ["fuck"],
    expectedArgs: "<user>",
    maxArgs: 1,
    cooldown: 300,
    callback: async (message: Discord.Message) => {
        if (message.channel.id != "831959023662137394") {
            return;
        }

        const member = message.mentions.members?.first() || message.member
        if (!member) {
            return;
        }

        const image = await getImage(message.author, "fuck")
        if (!image) {
            return message.reply("No \"fuck\" image found")
        }

        const embed = new Discord.EmbedBuilder()
            .setTitle(`${message.author.username} fucks ${member.displayName}`)
            .setImage(image)

        message.channel.send({embeds: [embed]})
    }
}
