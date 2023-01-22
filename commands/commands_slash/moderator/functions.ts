import {IServer} from "../../../models/server";
import Discord from "discord.js";
import {BotConfig, IBotConfig} from "../../../models/bot-config";

export enum LogType {
    Ban = "BAN",
    BotBan = "BOT BAN",
    SoftBan = "SoftBan",
    Kick = "KICK",
    Mute = "MUTE",
    Warn = "WARN",
    Infraction = "Infraction"
}

export async function addLog(
    server: IServer,
    logType: LogType,
    guild: Discord.Guild,
    member: Discord.GuildMember,
    moderator: Discord.User | Discord.GuildMember,
    reason: string = ""
) {
    const log = {
        caseNumber: server.moderation.length,
        guildId: guild.id,
        userId: member.id,
        logType: logType.toString(),
        moderatorId: moderator.id,
        reason: reason
    }

    const logChannel = await guild.channels.fetch(server.channels.logsChannel)
    if (logChannel && logChannel.isTextBased()) {
        const embed = new Discord.EmbedBuilder()
            .setTitle(`Case ${log.caseNumber} | ${log.logType} | ${member.user.tag}`)
            .setFields(
                {name: "User", value: member.toString(), inline: true},
                {name: "Moderator", value: moderator.toString(), inline: true},
                {name: "Reason", value: log.reason, inline: true}
            )
            .setFooter({text: `ID: ${log.userId}`})
            .setTimestamp()
        logChannel.send({embeds: [embed]})
    }

    server.moderation.push(log)
    await server.save()
    return true;
}

export async function warn(server: IServer, guild: Discord.Guild, channel: Discord.TextBasedChannel, member: Discord.GuildMember, moderator: Discord.User, reason: string) {
    const serverMsg = new Discord.EmbedBuilder()
        .setTitle(`User Warned`)
        .setDescription(`**${member} has been warned.\nReason: ${reason}**`)
        .setColor("#ff0000")

    const privateMsg = new Discord.EmbedBuilder()
        .setDescription(`You were warned in ${guild.name} for: ${reason}`)

    await Promise.all([
        addLog(server, LogType.Warn, guild, member, moderator, reason),
        channel.send({embeds: [serverMsg]}),
        member.user.send({embeds: [privateMsg]})
    ])
}

export async function mute(server: IServer, guild: Discord.Guild, channel: Discord.TextBasedChannel, member: Discord.GuildMember, moderator: Discord.User, duration: number, reason: string) {
    const serverMsg = new Discord.EmbedBuilder()
        .setTitle(`User Muted`)
        .setDescription(`**${member} has been muted.\nReason: ${reason}**`)
        .setColor("#ff0000")

    const privateMsg = new Discord.EmbedBuilder()
        .setDescription(`You were muted in ${guild.name} for: ${reason}`)

    await Promise.all([
        addLog(server, LogType.Mute, guild, member, moderator, reason),
        member.timeout(duration),
        channel.send({embeds: [serverMsg]}),
        member.user.send({embeds: [privateMsg]})
    ])
}

export async function ban(server: IServer, guild: Discord.Guild, channel: Discord.TextBasedChannel, member: Discord.GuildMember, moderator: Discord.User, reason: string) {
    const serverMsg = new Discord.EmbedBuilder()
        .setTitle("User Banned")
        .setDescription(`**${member} has been banned.\nReason: ${reason}**`)
        .setColor("#ff0000")

    const privateMsg = new Discord.EmbedBuilder()
        .setDescription(`You were banned in ${guild.name} for: ${reason}`)

    await Promise.all([
        addLog(server, LogType.Ban, guild, member, moderator, reason),
        member.user.send({embeds: [privateMsg]}),
        member.ban({deleteMessageSeconds: 604800, reason: reason}),
        channel.send({embeds: [serverMsg]}),
    ])
}

export async function isBlacklisted(member: Discord.GuildMember | Discord.User) {
    const botConfig: IBotConfig | null = await BotConfig.findOne<IBotConfig>().exec()
    return botConfig!.botBan.map(({userId}) => userId).includes(member.id)
}
