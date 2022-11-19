import Discord from "discord.js";
import path from "path";
import fs from "fs";
import {Zayden} from "../client";
import {ISlashCommand} from "./commands_slash/command";

const ignoreFiles = ["command.ts", "command_base.ts", "image_functions.ts"]

export function loadSlashCommands(client: Zayden) {
    client.slashCommands = new Discord.Collection();

    function readCommands(dir: string) {
        const files = fs.readdirSync(path.join(__dirname, dir)).filter(file => !ignoreFiles.includes(file));

        for (const file of files) {
            const stat = fs.lstatSync(path.join(__dirname, dir, file))
            if (stat.isDirectory()) {
                readCommands(path.join(dir, file))
                continue
            }

            const command: ISlashCommand = require(path.join(__dirname, dir, file))

            if ('data' in command && 'execute' in command) {
                client.slashCommands!.set(command.data.name, command);
            } else {
                console.log(`[WARNING] The command at ${file} is missing a required "data" or "execute" property.`);
            }
        }
    }

    readCommands("./commands_slash")

    if (client) {
        console.log(`Loaded ${client.slashCommands.size} slash commands`)
    }
}

export function loadMessageCommands(client: Zayden) {
    function readCommands(dir: string) {
        const files = fs.readdirSync(path.join(__dirname, dir)).filter(file => !ignoreFiles.includes(file));

        for (const file of files) {
            const stat = fs.lstatSync(path.join(__dirname, dir, file))
            if (stat.isDirectory()) {
                readCommands(path.join(dir, file))
                continue;
            }

            const command = require(path.join(__dirname, dir, file))

            if (client) {
                const commandBase = require("./commands_message/command_base")
                commandBase(client, command)
            }
        }
    }

    readCommands("./commands_message")
}
