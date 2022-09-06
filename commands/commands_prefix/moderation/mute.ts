import Discord from "discord.js";
import {IServer} from "../../../models/server";
import {addLog, LogType, setup} from "./functions";


module.exports = {
    commands: ["mute"],
    expectedArgs: "<user> <duration> <reason>",
    minArgs: 1,
    callback: async (message: Discord.Message, server: IServer, args: string[]) => {
        const {guild, member} = await setup(message, args)
        if (!guild) {
            return;
        }
        if (!member) {
            return message.reply("Member not found")
        }

        let reason = "No Reason Given";
        let duration = -1;
        if (args[1]) {
            const magnitude = Number(args[1].slice(0, -1))
            const delimiter = args[1].slice(-1)

            const durations: Record<string, number> = {
                's': 1,
                'm': 60,
                'h': 60 * 60,
                'd': 60 * 60 * 24
            }

            if (isNaN(magnitude) || !(delimiter in durations)) {
                reason = args.slice(1).join(" ")
            } else {
                duration = magnitude * durations[delimiter]
                reason = args.slice(2).join(" ")
            }
        }

        let mutedRole = (guild.roles.cache.find(role => role.name == "Muted") || await guild.roles.create({
            name: "Muted",
            color: "#818386"
        }));

        if (!mutedRole) {
            await message.reply("Failed to create muted role")
            return;
        }

        guild.channels.cache.forEach((channel) => {
            if (!(channel instanceof Discord.ThreadChannel)) {
                channel.permissionOverwrites.create(mutedRole, {
                    SendMessages: false,
                    Speak: false,
                    AddReactions: false
                })
            }
        })

        const serverMsg = new Discord.EmbedBuilder()
            .setTitle(`User Muted`)
            .setDescription(`<@${member.id}> has been muted by CK Staff for: ${reason}`)
            .setColor("#ff0000")

        const privateMsg = new Discord.EmbedBuilder()
            .setDescription(`You were muted in ${guild.name} for: ${reason}`)

        await addLog(server, LogType.Mute, guild, member, message.author, reason)

        await member.roles.add(mutedRole)

        message.channel.send({embeds: [serverMsg]});
        member.user.send({embeds: [privateMsg]}).catch(() => {
        });

        if (duration > 0) {
            setTimeout(() => {
                member.roles.remove(mutedRole);
            }, duration * 1000)
        }
    },
    permissions: ["MANAGE_MESSAGES"],
}
