import Discord from "discord.js"

module.exports = {
    commands: ["stats"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message: Discord.Message) => {
        const mentionedMember = message.mentions.members?.first()

        const username = mentionedMember?.displayName || message.member?.displayName || message.author.username;
        const userId = mentionedMember?.id || message.author.id

        const member_config = require(`../../user_configs/${userId}.json`);

        const embed = new Discord.MessageEmbed()
            .setTitle(`${username} Stats`)
            .addField("Number of Stars", member_config.number_of_stars.toString(), true)
            .addField("Given Stars", member_config.given_stars.toString(), true)
            .addField("Received Stars", member_config.received_stars.toString(), true)

        message.channel.send({embeds: [embed]})
    },
}