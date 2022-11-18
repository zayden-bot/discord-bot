import Discord from "discord.js";

export interface ISlashCommand {
    name: string
    data: any

    execute(interaction: Discord.Interaction): void
}
