import Discord from "discord.js"
import path from "path"
import fs from "fs"
import { Command } from "./command"

module.exports = (client?: Discord.Client) => {
    const ignoreFiles = [ "command_base.ts", "load_commands.ts", "command.ts" ]
    const commandBase = require(`./command_base`)

    const commands: Array<Command> = []

    const readCommands = (dir: string) => {
        const files = fs.readdirSync(path.join(__dirname, dir))
        
        for (const file of files) {
            if (ignoreFiles.includes(file)) { continue }

            const stat = fs.lstatSync(path.join(__dirname, dir, file))
            if (stat.isDirectory()) {
                readCommands(path.join(dir, file))
                continue
            }

            const options = require(path.join(__dirname, dir, file))
            if (options.commands && options.callback) {
                commands.push(options);
                if (client) { commandBase(client, options) }
            }

        }
    }

    readCommands("./")

    return commands
}
