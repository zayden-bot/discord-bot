import Discord from "discord.js";
import {ISlashCommand} from "./commands/commands_slash/command";
import {IMessageCommand} from "./commands/commands_message/command";

export class Zayden extends Discord.Client {
    version = [4, 2, 0]
    developers = ["211486447369322506"]
    activities: Discord.ActivityOptions[] = [
        {name: "College Kings", type: Discord.ActivityType.Playing, url: "https://www.patreon.com/collegekings"}
    ]
    slashCommands: Discord.Collection<string, ISlashCommand> = new Discord.Collection
    messageCommands: Discord.Collection<string, IMessageCommand> = new Discord.Collection

}
