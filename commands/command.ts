import Discord from "discord.js"

export interface Command {
    commands: string[]
    permissionError: string
    expectedArgs: string
    minArgs: number
    maxArgs: number
    cooldown: number
    permissions: Discord.PermissionResolvable[]
    requiredRoles: string[]
    callback: Function
}