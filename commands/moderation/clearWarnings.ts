import {IServer} from "../../models/server";
import Discord from "discord.js";
import {LogType, setup} from "./functions";

module.exports = {
    commands: ["clearwarnings", "clearwarns"],
    expectedArgs: "<user>",
    minArgs: 1,
    callback: async (message: Discord.Message, server: IServer, args: string[]) => {
        const {member} = await setup(message, args)
        if (!member) {
            await message.reply("Invalid member")
            return;
        }

        const startLength = server.moderation.length

        server.moderation = server.moderation.filter((log) => {
            return !(log.userId == member.id && log.logType == LogType.Warn)
        })
        await server.save()

        message.channel.send(`Cleared ${startLength - server.moderation.length} warnings from ${member.user.username}`)
    },
    permissions: ["MANAGE_MESSAGES"],
}