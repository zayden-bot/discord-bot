import {IServer} from "../../models/server";
import Discord from "discord.js";
import {LogType, setup} from "./functions";

module.exports = {
    commands: ["warnings"],
    expectedArgs: "<user>",
    minArgs: 1,
    callback: async (message: Discord.Message, server: IServer, args: string[]) => {
        const {member} = await setup(message, args)

        if (!member) {
            await message.reply("Please mention a valid member")
            return
        }

        let warningMsg = ""
        const warnings = server.moderation.filter(logs => logs.userId == member.id && logs.logType == LogType.Warn.toString())
        warnings.forEach(warning => {
            warningMsg += `**Case ${warning}**\n**Type:** ${warning.logType}\n**User:** <@${warning.userId}>\n**Moderator:** <@${warning.moderatorId}>\n**Reason:** ${warning.reason}\n\n`
        })

        if (warnings.length == 0) {
            await message.reply(`${member} has no warnings on record`)
            return;
        }
        
        const embed = new Discord.MessageEmbed()
            .setTitle(`Warnings for ${member.user.username}#${member.user.discriminator}`)
            .setDescription(warningMsg)
            .setColor("#ff0000")

        message.channel.send({embeds: [embed]})
    },
    permissions: ["MANAGE_MESSAGES"],
}