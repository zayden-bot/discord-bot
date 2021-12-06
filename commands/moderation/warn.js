const Discord = require("discord.js");
const moderation = require("../../moderationFunctions")

function addWarning(message, member, reason) {
    const serverMsg = new Discord.MessageEmbed()
    .setTitle(`User Warned`)
    .setDescription(`**<@${member.id}> has been warned by <@${message.author.id}>\nReason: ${reason}**`)
    .setColor("ff0000")

    const privateMsg = new Discord.MessageEmbed()
    .setDescription(`You were warned in ${message.guild.name} for: ${reason}`)

    moderation.addLog(message.guild, member, "warning", message.author, reason)

    message.channel.send(serverMsg)
    try { member.user.send(privateMsg) }
    catch {}
}

module.exports = {
    commands: ["warn"],
    expectedArgs: "<user> <reason>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const memberId = arguments[0].match(/\d+/)[0];
        const member = message.guild.members.cache.get(memberId)

        if (!member) {
            message.reply("Please mention a valid member")
            return
        }

        if (arguments[1]) { var reason = arguments.slice(1).join(" ") }
        else { var reason = "No Reason Given"}

        addWarning(message, member, reason)

        var warnings = moderation.getWarnings(message.guild, member)

        if (Object.keys(warnings).length > 1) {
            const muteMsg = new Discord.MessageEmbed()
            .setTitle(`${member.user.username} has been warned before:`)
            for (warning in warnings) {
                muteMsg.addField(`Case ${warning}`, `**Moderator:** <@${warnings[warning].moderator}>\n**Reason:** ${warnings[warning].reason}\n\n`)
            }
            message.channel.send(muteMsg)

            const filter = m => m.author.id === message.author.id
            message.channel.send("Would you like to increase the warning to a 1 hour mute? \`YES\` / \`NO\`").then(() => {
                message.channel.awaitMessages(filter, { max: 1, time: 30000, errors: ['time'] })
                .then(messages => {
                    const msg = messages.first()
                    if (msg.content.toUpperCase() == 'YES' || msg.content.toUpperCase() == 'Y') {
                        const mute = require("./mute")
                        mute.callback(message, [ `<@${member.id}>`, "1h", reason ], `<@${member.id}> 1h ${reason}`)
                        return
                    } else { throw "Warning Sent" }
                }).catch(messages => { addWarning(message, member, reason) })
            })
        }
    },
    permissions: ["MANAGE_MESSAGES"],
}