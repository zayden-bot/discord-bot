import Discord from "discord.js";
import {ISlashCommand} from "./commands/commands_slash/command";
import {IMessageCommand} from "./commands/commands_message/command";

export class Zayden extends Discord.Client {
    slashCommands: Discord.Collection<string, ISlashCommand> = new Discord.Collection
    messageCommands: Discord.Collection<string, IMessageCommand> = new Discord.Collection

}
