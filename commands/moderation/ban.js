const Discord = require("discord.js")

module.exports = {
    commands: ["ban"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        const member = message.mentions.members.first()
        const embed = new Discord.MessageEmbed()
            .setTitle(`User Banned`)
            .setDescription(`<@${member.id}> has been banned by CK Staff`)
            .setColor("ff0000")

        member.ban().then(
            message.channel.send(embed)
        )
    },
    permissions: "ADMINISTRATOR",
}