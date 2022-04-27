import {IServer} from "../../models/server";
import Discord from "discord.js";
import {setup} from "./functions";

module.exports = {
    commands: ["logs", "log"],
    expectedArgs: "<user>",
    minArgs: 1,
    callback: async (message: Discord.Message, server: IServer, args: string[]) => {
        const {member} = await setup(message, args)

        if (!member) {
            await message.reply("Please mention a valid member")
            return
        }

        let logMsg = ""
        const logs = server.moderation.filter(log => log.userId == member.id)

        for (const log of logs) {
            logMsg += `**Case ${log.caseNumber}**\n**Type:** ${log.logType}\n**User:** <@${log.userId}>\n**Moderator:** <@${log.moderatorId}>\n**Reason:** ${log.reason}\n\n`
        }

        const embed = new Discord.MessageEmbed()
            .setTitle(`Logs for ${member.user.username}#${member.user.discriminator}`)
            .setDescription(logMsg)
            .setColor("#ff0000")

        message.channel.send({embeds: [embed]})
    },
    permissions: ["MANAGE_MESSAGES"],
}