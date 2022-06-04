import Discord from "discord.js";
import {getImage} from "./image_cmd_base";

module.exports = {
    commands: ["goodnight", "gn"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: async (message: Discord.Message) => {
        const memberDisplayName = message.mentions.members?.first()?.displayName || message.member?.displayName || message.author.username

        const image = await getImage(message.author, "goodNight")
        if (!image) {
            return message.reply("No \"good night\" image found")
        }

        const embed = new Discord.MessageEmbed()
            .setTitle(`Good Night, ${memberDisplayName}`)
            .setImage(image)

        message.channel.send({embeds: [embed]})
            .catch(reason => {
                console.log("Failed to send goodnight message:", reason)
            })
    },
}
