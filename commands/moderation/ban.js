const Discord = require("discord.js");
const moderation = require("../../moderationFunctions")

module.exports = {
    commands: ["ban"],
    expectedArgs: "<user> <reason>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const memberId = arguments[0].match(/\d+/)[0];
        const member = message.guild.members.cache.get(memberId)

        if (!member) {
            message.reply("Please mention a valid member")
            return
        }

        if (arguments[1]) { var reason = arguments.slice(1).join(" ") }
        else { var reason = "No Reason Given"}

        const serverMsg = new Discord.MessageEmbed()
        .setTitle(`User Banned`)
        .setDescription(`<@${member.id}> has been banned by CK Staff`)
        .setColor("ff0000")

        const privateMsg = new Discord.MessageEmbed()
        .setDescription(`You were banned in ${message.guild.name} for: ${reason}`)

        moderation.addLog(message.guild, member, "ban", message.author, reason)

        member.ban( {days: 7, reason: reason} )
        .then( () => {
            message.channel.send(serverMsg)
            try { member.user.send(privateMsg) }
            catch {}
        })
        .catch( err => { message.reply(`Failed to ban ${member.user.username}`) })
    },
    permissions: ["BAN_MEMBERS"],
}