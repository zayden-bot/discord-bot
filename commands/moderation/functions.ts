import {IServer} from "../../models/server";
import Discord from "discord.js";
import {parseId} from "../../common";
import {BotConfig, IBotConfig} from "../../models/bot-config";

interface ISetup {
    guild: Discord.Guild | undefined,
    member: Discord.GuildMember | undefined,
    reason: string
}

export enum LogType {
    Ban = "BAN",
    BotBan = "BOT BAN",
    Mute = "MUTE",
    Warn = "WARN"
}

export async function setup(message: Discord.Message, args: string[]) {
    let rv: ISetup = {
        guild: message.guild || undefined,
        member: undefined,
        reason: args[1] ? args.slice(1).join(" ") : "No Reason Given"
    }

    const userId = parseId(args[0])
    if (!userId) {
        return rv
    }
    rv.member = await message.guild?.members.fetch(userId) || undefined

    return rv
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
    const botConfig: IBotConfig = await BotConfig.findOne().exec()
    return botConfig.botBan.map(({userId}) => userId).includes(member.id)
}