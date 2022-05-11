import {IServer} from "../../models/server";
import Discord from "discord.js";
import {addLog, LogType, setup} from "./functions";

module.exports = {
    commands: ["warn"],
    expectedArgs: "<user> <reason>",
    minArgs: 1,
    callback: async (message: Discord.Message, server: IServer, args: string[]) => {
        const {guild, member, reason} = await setup(message, args)
        if (!guild) {
            return;
        }
        if (!member) {
            await message.reply("Please mention a valid member")
            return
        }

        await addLog(server, LogType.Warn, guild, member, message.author, reason)

        const serverMsg = new Discord.MessageEmbed()
            .setTitle(`User Warned`)
            .setDescription(`**<@${member.id}> has been warned by <@${message.author.id}>\nReason: ${reason}**`)
            .setColor("#ff0000")

        const privateMsg = new Discord.MessageEmbed()
            .setDescription(`You were warned in ${guild.name} for: ${reason}`)

        message.channel.send({embeds: [serverMsg]})
        member.user.send({embeds: [privateMsg]})
            .catch()

        const warnings = server.moderation.filter(log => log.userId == member.id && log.logType == LogType.Warn.toString())

        if (warnings.length > 1) {
            const muteMsg = new Discord.MessageEmbed()
                .setTitle(`${member.user.username} has been warned before:`)
            for (const warning of warnings) {
                muteMsg.addField(`Case ${warning.caseNumber}`, `**Moderator:** <@${warning.moderatorId}>\n**Reason:** ${warning.reason}\n\n`)
            }
            message.channel.send({embeds: [muteMsg]})

            const filter = ((m: Discord.Message) => m.author.id === message.author.id)
            message.channel.send("Would you like to increase the warning to a 1 hour mute? \"YES\" / \"NO\"")

            message.channel.awaitMessages({filter, max: 1, time: 30000, errors: ['time']})
                .then(async messages => {
                    const msg = messages.first()

                    if (msg?.content.toUpperCase().startsWith('Y')) {
                        const mute = require("./mute")
                        await mute.callback(message, server, [member.id.toString(), "1h", reason])
                    } else {
                        await message.reply("Warning Sent")
                    }
                })
                .catch(async () => {
                    await message.reply("Warning sent (response timed out)")
                })
        }
    },
    permissions: ["MANAGE_MESSAGES"],
}