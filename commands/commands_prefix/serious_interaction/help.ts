import Discord from "discord.js"
import {client} from "../../../index";

module.exports = {
    commands: ["help", "h", "?"],
    description: "Zayden Help Command",
    callback: (message: Discord.Message) => {
        if (!message.member) {
            return;
        }

        const {prefix} = require("../../../configs/bot_config.json")

        let reply = "Zayden's Commands:\n"

        for (let command of client.slashCommands) {
            let permissions = command.permissions
            let roles = command.requiredRoles
            let hasPermission = true

            // Check for permissions
            if (permissions) {
                if (!message.guild) {
                    continue
                }

                if (Symbol.iterator in Object(permissions)) {
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
                if (message.guild == null) {
                    continue
                }
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
