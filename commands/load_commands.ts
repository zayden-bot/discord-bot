import Discord from "discord.js"
import path from "path"
import {Command} from "./commands_message/command"
import fs from "fs";

module.exports = (client?: Discord.Client) => {
    const ignoreFiles = ["command_base.ts", "load_commands.ts", "command.ts"]

    const commands: Array<Command> = []

    function readCommands(dir: string) {
        const files = fs.readdirSync(path.join(__dirname, dir))

        for (const file of files.filter((file) => !ignoreFiles.includes(file))) {

            const stat = fs.lstatSync(path.join(__dirname, dir, file))
            if (stat.isDirectory()) {
                readCommands(path.join(dir, file))
                continue
            }

            const options = require(path.join(__dirname, dir, file))
            const parentDir = dir.split(path.sep)[0]

            if ((options.commands || options.command) && options.callback) {
                if (parentDir == "commands_prefix") {
                    commands.push(options);
                }

                const commandBase = require(`./${parentDir}/command_base`)
                if (client) {
                    commandBase(client, options)
                }
            }
        }
    }

    readCommands("./")

    if (client) {
        console.log(`Loaded ${commands.length} commands`)
    }
    return commands
}
