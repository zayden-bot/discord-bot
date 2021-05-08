const Discord = require("discord.js");
const moderation = require("../../moderationFunctions")

module.exports = {
    commands: ["mute"],
    expectedArgs: "<user> <duration> <reason>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const memberId = arguments[0].match(/\d+/)[0];
        const member = message.guild.members.cache.get(memberId)

        if (!member) {
            message.reply("Please mention a valid member")
            return
        }

        let reason = "No Reason Given"
        let duration = arguments[1]
        const durations = {
            s: 1,
            m: 60,
            h: 60 * 60,
            d: 60 * 60 * 24,
        }

        try {
            if (isNaN(duration.slice(0, -1)) || !(duration.slice(-1) in durations)) {
                duration = -1
                try { reason = arguments.slice(1).join(" ") }
                catch {}
            } else {
                duration = duration.slice(0, -1) * durations[duration.slice(-1)]
                try { reason = arguments.slice(2).join(" ") }
                catch {}
            }
        } catch { duration = -1 }

        let mutedRole = message.guild.roles.cache.find(role => role.name == "Muted")
        if (!mutedRole) {
            // Create role and deny permission to speak in all channels
            mutedRole = message.guild.roles.create({
                data: {
                    name: "Muted",
                    color: "#818386",
                    permissions: []
                }
            }).then( (role) => {
                message.guild.channels.cache.forEach( (channel, id) => {
                    channel.createOverwrite(role, {
                        SEND_MESSAGES: false,
                        SPEAK: false,
                        ADD_REACTIONS: false
                    })
                })
            })
        }

        const serverMsg = new Discord.MessageEmbed()
        .setTitle(`User Muted`)
        .setDescription(`<@${member.id}> has been muted by CK Staff for: ${reason}`)
        .setColor("ff0000")

        const privateMsg = new Discord.MessageEmbed()
        .setDescription(`You were muted in ${message.guild.name} for: ${reason}`)

        moderation.addLog(message.guild, member, "muted", message.author, reason)

        member.roles.add(mutedRole)

        message.channel.send(serverMsg)
        try { member.user.send(privateMsg) }
        catch (error) {
            if (!(error instanceof(Discord.DiscordAPIError))) {
                throw error
            }
        }

        if (duration > 0) {
            setTimeout( () => {
                member.roles.remove(mutedRole);
            }, duration * 1000)
        }

    },
    permissions: ["MANAGE_MESSAGES"],
}