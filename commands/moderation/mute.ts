import Discord from "discord.js";


module.exports = {
    commands: ["mute"],
    expectedArgs: "<user> <duration> <reason>",
    minArgs: 1,
    callback: (message: Discord.Message, args: string[], text: string) => {
        if (!message.guild) { return message.reply("Invalid Guild"); }

        const match = args[0].match(/\d+/)
        if (!match || !match[0]) { return message.reply("You must enter a valid member"); }

        const member = message.guild.members.cache.get(match[0])
        if (!member) { return message.reply("Member not found") }

        let reason: string;
        let duration: number;

        if (args[1]) {
            const magnitude = Number(args[1].slice(0, -1))
            const delimeter = args[1].slice(-1)

            const durations: Record<string, number> = {
                's': 1,
                'm': 60,
                'h': 60 * 60,
                'd': 60 * 60 * 24
            }

            if (isNaN(magnitude) || !(delimeter in durations)) {
                duration = -1
                reason = args.slice(1).join(" ")
            } else {
                duration = magnitude * durations[delimeter]
                reason = args.slice(2).join(" ")
            }

        } else {
            duration = -1
            reason = "No reason given"
        }

        let mutedRole: any = message.guild.roles.cache.find(role => role.name == "Muted")
        if (!mutedRole) {
            // Create role and deny permission to speak in all channels
            mutedRole = message.guild.roles.create({
                name: "Muted",
                color: "#818386",
                permissions: []
            }).then((role) => {
                message.guild?.channels.cache.forEach( (channel) => {
                    if (channel.type == "GUILD_TEXT" || channel.type == "GUILD_VOICE") {
                        channel.permissionOverwrites.create(role, {
                            SEND_MESSAGES: false,
                            SPEAK: false,
                            ADD_REACTIONS: false
                        })
                    }
                })
            })
        }

        const serverMsg = new Discord.MessageEmbed()
        .setTitle(`User Muted`)
        .setDescription(`<@${member.id}> has been muted by CK Staff for: ${reason}`)
        .setColor("#ff0000")

        const privateMsg = new Discord.MessageEmbed()
        .setDescription(`You were muted in ${message.guild.name} for: ${reason}`)
        
        const moderation = require("../../moderationFunctions")
        moderation.addLog(message.guild, member, "muted", message.author, reason)

        member.roles.add(mutedRole)

        message.channel.send({embeds: [serverMsg]});
        member.user.send({embeds: [privateMsg]})
        .catch(() => {});

        if (duration > 0) {
            setTimeout( () => {
                member.roles.remove(mutedRole);
            }, duration * 1000)
        }
    },
    permissions: ["MANAGE_MESSAGES"],
}