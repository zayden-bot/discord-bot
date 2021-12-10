import Discord from "discord.js"

module.exports = {
    commands: ["help", "h", "?"],
    description: "Zayden Help Command",
    callback: (message: Discord.Message, args: string[], text: string) => {
        if (!message.member) { return; }

        const { prefix } = require("../../configs/bot_config.json")

        let reply = "Zayden's Commands:\n"

        const loadCommands = require("../load_commands")
        const commands = loadCommands()

        for (let command of commands) {
            let permissions = command.permissions
            let roles = command.requiredRoles
            let hasPermission = true

            // Check for permissions
            if (permissions) {
                if (message.guild == null) { continue }

                if (typeof permissions === "string") {
                    permissions = [permissions]
                }

                for (let permission of permissions) {
                    if (!message.member.permissions.has(permission)) {
                        hasPermission = false
                        break
                    }
                }
            }

            // Check for requiredRoles
            if (roles) {
                if (message.guild == null) { continue }
                if (typeof roles === "string") {
                    roles = [roles]
                }

                for (let requiredRole of roles) {
                    const role = message.guild.roles.cache.find(role => role.name === requiredRole)

                    if (!role || !message.member.roles.cache.has(role.id)) {
                        hasPermission = false
                        break
                    }
                }
            }

            if (!hasPermission) {
                continue
            }
            
            const mainCommand = typeof command.commands === "string" ? command.commands : command.commands[0]
            const args = command.expectedArgs ? ` ${command.expectedArgs}` : ""
            
            reply += `**${prefix}${mainCommand}${args}**\n`
            
        }

        message.channel.send(reply)
    },
}
