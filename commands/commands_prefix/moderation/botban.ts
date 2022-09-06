import Discord from "discord.js";
import {IServer} from "../../../models/server";
import {BotConfig, IBotConfig} from "../../../models/bot-config";
import {LogType, setup} from "./functions";

module.exports = {
    commands: ["botban", "addbotban"],
    expectedArgs: "<user>",
    minArgs: 1,
    callback: async (message: Discord.Message, server: IServer, args: string[]) => {
        const {member, reason} = await setup(message, args)

        if (!member) {
            await message.reply("Invalid member")
            return;
        }

        const botConfigJson = require("../../../configs/bot_config.json")
        if (botConfigJson.developers.includes(member.id)) {
            await message.reply("This is a protected member and cannot be bot banned")
            return;
        }

        const botConfig: IBotConfig | null = await BotConfig.findOne<IBotConfig>().exec()
        const botBans = new Set(botConfig!.botBan)
            .add({
                caseNumber: botConfig!.botBan.length,
                userId: member.id,
                logType: LogType.BotBan.toString(),
                moderatorId: message.author.id,
                reason: reason
            })
        botConfig!.botBan = Array.from(botBans)

        await Promise.all([
            botConfig!.save(),
            message.reply(`Successfully bot banned ${member}`)
        ])
    },
    permissions: ["MANAGE_MESSAGES"],
}
