import Discord from "discord.js"
import {IServer} from "../../../models/server";

// Template Command
module.exports = {
    commands: ["add", "addition"],
    expectedArgs: "<num1> <num2> <num3> ...",
    permissionError: "",
    minArgs: 2,
    callback: async (message: Discord.Message, server: IServer, args: string[]) => {
        let sum = 0
        for (const arg of args) {
            if (isNaN(Number(arg))) {
                await message.reply("Invalid arguments")
                return;
            }
            sum += Number(arg);
        }

        await message.reply(`Answer: ${sum}`)
    },
    permissions: ["MANAGE_MESSAGES"],
    requiredRoles: ["Staff"],
}