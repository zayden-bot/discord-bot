import Discord from "discord.js"
import {getUserConfig, IUserConfig} from "../../models/user-config";

module.exports = {
    commands: ["stats"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: async (message: Discord.Message) => {
        const mentionedMember = message.mentions.members?.first()

        const username = mentionedMember?.displayName || message.member?.displayName || message.author.username;
        const userId = mentionedMember?.id || message.author.id

        const member_config: IUserConfig = await getUserConfig(userId);

        const embed = new Discord.MessageEmbed()
            .setTitle(`${username} Stats`)
            .addField("Number of Stars", member_config.stars.numberOfStars.toString(), true)
            .addField("Given Stars", member_config.stars.givenStars.toString(), true)
            .addField("Received Stars", member_config.stars.receivedStars.toString(), true)

        message.channel.send({embeds: [embed]})
            .catch(reason => {
                console.log("Failed to send stats message:", reason)
            })
    },
}