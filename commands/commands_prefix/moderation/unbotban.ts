import Discord from "discord.js";
import {IServer} from "../../../models/server";
import {setup} from "./functions";
import {BotConfig, IBotConfig} from "../../../models/bot-config";

module.exports = {
    commands: ["unbotban", "removebotban"],
    expectedArgs: "<user>",
    minArgs: 1,
    callback: async (message: Discord.Message, server: IServer, args: string[]) => {
        const {member} = await setup(message, args)
        if (!member) {
            await message.reply("Please mention a valid user.")
            return
        }

        const botConfig: IBotConfig = await BotConfig.findOne().exec()
        botConfig.botBan = botConfig.botBan.filter((banLog) => banLog.userId != member.id)

        await Promise.all([
            botConfig.save(),
            message.reply(`Successfully Removed Bot Ban from ${member}`)
        ])
    },
    permissions: ["MANAGE_MESSAGES"],
}