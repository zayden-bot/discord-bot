const Discord = require("discord.js");
const moderation = require("../../moderationFunctions")

module.exports = {
    commands: ["logs", "log"],
    expectedArgs: "<user>",
    minArgs: 1,
    maxArgs: 1,
    callback: (message, arguments, text) => {
        const memberId = arguments[0].match(/\d+/)[0];
        const member = message.guild.members.cache.get(memberId)
        
        if (!member) {
            message.reply("Please mention a valid member")
            return
        }

        let logMsg = ""
        const logs = moderation.getLogs(message.guild, member)

        for (key in logs) {
            let type = logs[key].type
            type = type.charAt(0).toUpperCase() + type.slice(1) // Capatalise first character
            logMsg += `**Case ${key}**\n**Type:** ${type}\n**User:** <@${logs[key].userId}>\n**Moderator:** <@${logs[key].moderator}>\n**Reason:** ${logs[key].reason}\n\n`
        }

        const embed = new Discord.MessageEmbed()
        .setTitle(`Logs for ${member.user.username}#${member.user.discriminator}`)
        .setDescription(logMsg)
        .setColor("ff0000")

        message.channel.send({embeds: [embed]})
    },
    permissions: ["MANAGE_MESSAGES"],
}