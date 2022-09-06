import Discord from "discord.js"
import {getUserConfig, IUserConfig} from "../../../models/user-config";

module.exports = {
    commands: ["stats"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: async (message: Discord.Message) => {
        const mentionedMember = message.mentions.members?.first()

        const username = mentionedMember?.displayName || message.member?.displayName || message.author.username;
        const userId = mentionedMember?.id || message.author.id

        const member_config: IUserConfig = await getUserConfig(userId);

        const embed = new Discord.EmbedBuilder()
            .setTitle(`${username} Stats`)
            .addFields([
                {name: "Number of Stars", value: member_config.stars.numberOfStars.toString(), inline: true},
                {name: "Given Stars", value: member_config.stars.givenStars.toString(), inline: true},
                {name: "Received Stars", value: member_config.stars.receivedStars.toString(), inline: true}
            ])

        message.channel.send({embeds: [embed]})
            .catch(reason => {
                console.log("Failed to send stats message:", reason)
            })
    },
}
