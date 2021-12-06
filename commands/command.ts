import Discord from "discord.js"

export interface Command {
    commands: string | string[]
    permissionError: string
    expectedArgs: string
    minArgs: number
    maxArgs: number
    cooldown: number
    permissions: Array<Discord.PermissionResolvable>
    requiredRoles: string | string[]
    callback: Function
}