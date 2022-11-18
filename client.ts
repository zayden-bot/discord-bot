import Discord from "discord.js";
import {ISlashCommand} from "./commands/commands_slash/command";

export class Zayden extends Discord.Client {
    slashCommands: Discord.Collection<string, ISlashCommand> = new Discord.Collection

}
