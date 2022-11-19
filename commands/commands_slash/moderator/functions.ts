import {IServer} from "../../../models/server";
import Discord from "discord.js";
import {BotConfig, IBotConfig} from "../../../models/bot-config";

export enum LogType {
    Ban = "BAN",
    BotBan = "BOT BAN",
    Mute = "MUTE",
    Warn = "WARN"
}

export async function addLog(
    server: IServer,
    logType: LogType,
    guild: Discord.Guild,
    user: Discord.User | Discord.GuildMember,
    moderator: Discord.User | Discord.GuildMember,
    reason: string = ""
) {
    reason = reason.replace("'", "\'")

    const log = {
        caseNumber: server.moderation.length,
        guildId: guild.id,
        userId: user.id,
        logType: logType.toString(),
        moderatorId: moderator.id,
        reason: reason
    }

    server.moderation.push(log)
    await server.save()
    return true;
}

export async function isBlacklisted(member: Discord.GuildMember | Discord.User) {
    const botConfig: IBotConfig | null = await BotConfig.findOne<IBotConfig>().exec()
    return botConfig!.botBan.map(({userId}) => userId).includes(member.id)
}
