import Discord from "discord.js";
import {getImage} from "./image_cmd_base";

module.exports = {
    commands: ["goodnight", "gn"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: async (message: Discord.Message) => {
        const memberDisplayName = message.mentions.members?.first()?.displayName || message.member?.displayName || message.author.username

        const image = await getImage(message.author, "goodNight")

        const embed = new Discord.EmbedBuilder()
            .setTitle(`Good Night, ${memberDisplayName}`)
            .setImage(image)

        message.channel.send({embeds: [embed]})
            .catch(reason => {
                console.log("Failed to send goodnight message:", reason)
            })
    },
}
