const Discord = require("discord.js");
const moderation = require("../../moderationFunctions")

module.exports = {
    commands: ["warn"],
    expectedArgs: "<user> <reason>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const member = message.mentions.members.first()
        if (!member) {
            message.reply("Please mention a valid member")
            return
        }

        if (arguments[1]) { var reason = arguments.slice(1).join(" ") }
        else { var reason = "No Reason Given"}

        const serverMsg = new Discord.MessageEmbed()
        .setTitle(`User Warned`)
        .setDescription(`**<@${member.id}> has been warned by <@${message.author.id}>\nReason: ${reason}**`)
        .setColor("ff0000")

        const privateMsg = new Discord.MessageEmbed()
        .setDescription(`You were warned in ${message.guild.name} for: ${reason}`)

        moderation.addLog(message.guild, member, "warning", message.author, reason)

        message.channel.send(serverMsg)
        member.user.send(privateMsg)
    },
    requiredRoles: ["Security"],
}