import Discord from "discord.js";
import {IModLog} from "../../../models/server_settings/ModLogSchema";
import {getConnection} from "../../../servers";
import {IChannel} from "../../../models/server_settings/ChannelSchema";

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
    guildId: string,
    logType: LogType,
    member: Discord.GuildMember,
    moderatorId: string,
    reason: string = ""
) {

    const conn = getConnection(guildId)
    const modLogs = conn.model<IModLog>("modLogs")

    const log: IModLog = {
        logId: await modLogs.count(),
        logType: logType,
        moderatorId: moderatorId,
        reason: reason,
        userId: member.id
    }

    await (await modLogs.create(log)).save()

    const embed = new Discord.EmbedBuilder()
        .setTitle(`Case ${log.logId} | ${log.logType} | ${member.user.tag}`)
        .setFields(
            {name: "User", value: member.toString(), inline: true},
            {name: "Moderator", value: `<@${moderatorId}>`, inline: true},
            {name: "Reason", value: log.reason, inline: true}
        )
        .setFooter({text: `ID: ${log.userId}`})
        .setTimestamp()

    const logChannels = await conn.model<IChannel>("Channels").find({category: "log"})
    for (const logChannel of logChannels) {
        const channel = await member.guild.channels.fetch(logChannel.id)

        if (channel?.type == Discord.ChannelType.GuildText)
            channel.send({embeds: [embed]})
    }
}

export async function warn(guild: Discord.Guild, channel: Discord.TextBasedChannel, member: Discord.GuildMember, moderator: Discord.User, reason: string = "") {
    const serverMsg = new Discord.EmbedBuilder()
        .setTitle(`User Warned`)
        .setDescription(`**${member} has been warned.\nReason: ${reason}**`)
        .setColor("#ff0000")

    const privateMsg = new Discord.EmbedBuilder()
        .setDescription(`You were warned in ${guild.name} for: ${reason}`)

    await Promise.all([
        addLog(guild.id, LogType.Warn, member, moderator.id, reason),
        channel.send({embeds: [serverMsg]}),
        member.user.send({embeds: [privateMsg]})
    ])
}

export async function mute(guild: Discord.Guild, channel: Discord.TextBasedChannel, member: Discord.GuildMember, moderator: Discord.User, duration: number, reason: string) {
    const serverMsg = new Discord.EmbedBuilder()
        .setTitle(`User Muted`)
        .setDescription(`**${member} has been muted.\nReason: ${reason}**`)
        .setColor("#ff0000")

    const privateMsg = new Discord.EmbedBuilder()
        .setDescription(`You were muted in ${guild.name} for: ${reason}`)

    await Promise.all([
        addLog(guild.id, LogType.Mute, member, moderator.id, reason),
        member.timeout(duration),
        channel.send({embeds: [serverMsg]}),
        member.user.send({embeds: [privateMsg]})
    ])
}

export async function ban(guild: Discord.Guild, channel: Discord.TextBasedChannel, member: Discord.GuildMember, moderator: Discord.User, reason: string) {
    const serverMsg = new Discord.EmbedBuilder()
        .setTitle("User Banned")
        .setDescription(`**${member} has been banned.\nReason: ${reason}**`)
        .setColor("#ff0000")

    const privateMsg = new Discord.EmbedBuilder()
        .setDescription(`You were banned in ${guild.name} for: ${reason}`)

    await Promise.all([
        addLog(guild.id, LogType.Ban, member, moderator.id, reason),
        member.user.send({embeds: [privateMsg]}),
        member.ban({deleteMessageSeconds: 604800, reason: reason}),
        channel.send({embeds: [serverMsg]}),
    ])
}
