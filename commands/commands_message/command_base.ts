// noinspection DuplicatedCode

import Discord from "discord.js"
import {Command} from "./command"
import {IServer, Server} from "../../models/server";
import {isBlacklisted} from "../commands_prefix/moderation/functions";

let recentlyRan: string[] = []

module.exports = (client: Discord.Client, commandOptions: Command) => {
    let {
        command,
        permissionError = "You do not have permission to run this command",
        cooldown = -1,
        permissions = [],
        requiredRoles = [],
        callback
    } = commandOptions

    if (permissionError.length == 0) {
        permissionError = "You do not have permission to run this command"
    }

    // noinspection SuspiciousTypeOfGuard
    if (typeof (requiredRoles) == "string") {
        requiredRoles = [requiredRoles]
    }

    client.on("messageCreate", async message => {
        const {member, guild} = message

        if (!member || member.user.bot || !guild) {
            return
        }

        const botConfig = require("../../configs/bot_config.json");
        const server: IServer | null = await Server.findOne<IServer>({id: guild.id}).exec()
        if (!server) return;

        // Check if the command is enabled in that server
        if (server.disabledCommands.includes(command) && !botConfig.developers.includes(member.id)) {
            return
        }

        // Check if the user has the correct permissions to run the command
        for (const permission of permissions) {
            if (guild && !member.permissions.has(permission) && !botConfig.developers.includes(member.id)) {
                await message.reply({content: permissionError})
                return
            }
        }

        // Check if the user has the required roles to run the command
        const roles: Discord.Role[] = []
        for (const requiredRole of requiredRoles) {
            const role = guild.roles.cache.find(role => role.name === requiredRole)
            if (role) {
                roles.push(role)
            }
        }

        if (roles.length > 0 && !member.roles.cache.hasAny(...roles.map(role => role.id)) && !botConfig.developers.includes(member.id)) {
            await message.reply({content: permissionError})
            return
        }

        // Check if the user is blacklisted
        if (guild && await isBlacklisted(member) && !botConfig.developers.includes(member.id)) {
            return
        }

        // Check if the command is on cooldown
        let cooldownString: string;
        try {
            cooldownString = `${guild.id}-${member.id}-${command}`;
        } catch {
            cooldownString = `privateMessage-${message.author.id}-${command}`;
        }

        if (cooldown > 0 && recentlyRan.includes(cooldownString) && !member.roles.cache.has(server.roles.moderationRole)) {
            const reply = await message.reply({content: `You can only use this command once per ${cooldown} seconds`}).catch(error => console.log(error))
            if (reply) {
                setTimeout(() => reply.delete(), 5000)
            }
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

        callback(message, server)

        return
    }).setMaxListeners(0)
}
