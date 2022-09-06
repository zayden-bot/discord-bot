import Discord from "discord.js"
import {IServer} from "../../../models/server";
import {addLog, LogType, setup} from "./functions";

module.exports = {
    commands: ["ban"],
    expectedArgs: "<user> <reason>",
    minArgs: 1,
    callback: async (message: Discord.Message, server: IServer, args: string[]) => {
        const {guild, member, reason} = await setup(message, args)
        if (!guild) {
            return;
        }
        if (!member) {
            await message.reply("Invalid member")
            return;
        }

        const serverMsg = new Discord.EmbedBuilder()
            .setTitle(`User Banned`)
            .setDescription(`${member} has been banned by College Kings Staff`)
            .setColor("#ff0000")

        const privateMsg = new Discord.EmbedBuilder()
            .setDescription(`You were banned in ${guild.name} for:\n${reason}`)

        await addLog(server, LogType.Ban, guild, member, message.author, reason)

        Promise.all([
            member.ban({deleteMessageDays: 7, reason: reason}),
            message.channel.send({embeds: [serverMsg]}),
            member.user.send({embeds: [privateMsg]})
        ]).catch(() => {
            message.reply(`Failed to ban ${member.user.username}`)
        })
    },
    permissions: ["BAN_MEMBERS"],
}
