import Discord from "discord.js"
import { servers } from "../server"
import { Command } from "./command"

let recentlyRan: string[] = []

module.exports = (client: Discord.Client, commandOptions: Command) => {
    let {
        commands,
        expectedArgs = "",
        permissionError = "You do not have permission to run this command",
        minArgs = 0,
        maxArgs = null,
        cooldown = -1,
        permissions = [],
        requiredRoles = [],
        callback
    } = commandOptions

    if (typeof (commands) == "string") {
        commands = [commands]
    }

    if (permissionError.length == 0) {
        permissionError = "You do not have permission to run this command"
    }

    if (typeof (requiredRoles) == "string") {
        requiredRoles = [requiredRoles]
    }

    client.on("messageCreate", message => {
        const { member, content, guild, channel } = message

        if (member == null || guild == null) { return }

        const server = servers[guild.id]

        const botConfig = require("../configs/bot_config.json");

        for (const alias of commands) {
            if (content.split(" ")[0].toLowerCase() == `${botConfig.prefix}${alias.toLowerCase()}`) {

                // Check if the command is enabled in that server
                if (server.disabledCommands.includes(commands[0])) { return }

                // Check if the user has the correct permissions to run the command
                for (const permission of permissions) {
                    if (guild && !member.permissions.has(permission) && !botConfig.developers.includes(member.id)) {
                        message.reply({ content: permissionError })
                        return
                    }
                }

                // Check if the user has the required roles to run the command
                const roles: Discord.Role[] = []
                for (const requiredRole of requiredRoles) {
                    const role = guild.roles.cache.find(role => role.name === requiredRole)
                    if (role) { roles.push(role) }
                }

                if (roles.length > 0 && !member.roles.cache.hasAny(...roles.map(role => role.id)) && !botConfig.developers.includes(member.id)) {
                    message.reply({ content: permissionError })
                    return
                }

                // Check if the user is blacklisted
                const blacklist = require("../blacklist");
                if (guild && blacklist.isBlacklisted(member.id) && !botConfig.developers.includes(member.id)) {
                    return
                }

                // Check if the command is on cooldown
                try { var cooldownString = `${guild.id}-${member.id}-${commands[0]}` }
                catch { var cooldownString = `privateMessage-${message.author.id}-${commands[0]}` }
                if (cooldown > 0 && recentlyRan.includes(cooldownString) && !member.roles.cache.has(server.roles.moderationRole)) {
                    message.reply("You cannot use that command so soon, please wait")
                    return
                }

                // Create the arguments variable
                const args = content.split(/[ ]+/)
                args.shift()

                // Check if the user inputed the correct number of arguments
                if (args.length < minArgs || (maxArgs !== null && args.length > maxArgs)) {
                    const embed = new Discord.MessageEmbed()
                        .setColor("#ff0000")
                        .setDescription(`Invalid command usage, try using it like:\n\`${botConfig.prefix}${alias} ${expectedArgs}\``)

                    channel.send({ embeds: [embed] })
                        .then(msg => {
                            setTimeout(() => message.delete().catch((err: any) => { console.log(err) }), 5000)
                            setTimeout(() => msg.delete(), 5000);
                        });
                    return
                }

                // Add command to recentlyRan if command has cooldown
                if (cooldown > 0) {
                    recentlyRan.push(cooldownString)

                    setTimeout(() => {
                        recentlyRan = recentlyRan.filter((string) => {
                            return string !== cooldownString
                        })
                    }, 1000 * cooldown);
                }

                console.log(`Running ${botConfig.prefix}${alias}`)
                callback(message, args, args.join(" "))

                return
            }
        }
    }).setMaxListeners(0)
}
