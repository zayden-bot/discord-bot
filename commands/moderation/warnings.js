const Discord = require("discord.js");
const moderation = require("../../moderationFunctions")

module.exports = {
    commands: ["warnings"],
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

        let warningMsg = ""
        const warnings = moderation.getWarnings(message.guild, member)

        for (warning in warnings) {
            let type = warnings[warning].type
            type = type.charAt(0).toUpperCase() + type.slice(1) // Capatalise first character
            warningMsg += `**Case ${warning}**\n**Type:** ${type}\n**User:** <@${warnings[warning].userId}>\n**Moderator:** <@${warnings[warning].moderator}>\n**Reason:** ${warnings[warning].reason}\n\n`
        }

        const embed = new Discord.MessageEmbed()
        .setTitle(`Warnings for ${member.user.username}#${member.user.discriminator}`)
        .setDescription(warningMsg)
        .setColor("ff0000")

        message.channel.send({embeds: [embed]})
    },
    permissions: ["MANAGE_MESSAGES"],
}