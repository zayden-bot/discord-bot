import Discord from "discord.js"

export interface Command {
    command: string
    permissionError: string
    cooldown: number
    permissions: Discord.PermissionResolvable[]
    requiredRoles: string | string[]
    callback: Function
}