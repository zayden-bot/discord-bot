const Discord = require("discord.js");
const blacklist = require("../../blacklist");

module.exports = {
    commands: ["ban"],
    expectedArgs: "<user>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const member = message.mentions.members.first()

        if (member) {
            const embed = new Discord.MessageEmbed()
            .setTitle(`User Banned`)
            .setDescription(`<@${member.id}> has been banned by CK Staff`)
            .setColor("ff0000")

            member.ban( {days: 7, reason: text} )
            .then( () => { message.channel.send(embed) })
            .catch( err => { message.reply(`Failed to ban ${member.user.username}`) })
        }
    },
    permissions: "BAN_MEMBERS",
}