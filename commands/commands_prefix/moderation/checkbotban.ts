import {isBlacklisted, setup} from "./functions";
import Discord from "discord.js";
import {IServer} from "../../../models/server";

module.exports = {
    commands: "checkbotban",
    expectedArgs: "<user>",
    minArgs: 1,
    callback: async (message: Discord.Message, server: IServer, args: string[]) => {
        const {member} = await setup(message, args)
        if (!member) {
            await message.reply("Invalid member")
            return;
        }


        if (await isBlacklisted(member)) {
            await message.reply("The user is blacklisted!");
        } else {
            await message.reply("The user is not blacklisted!");
        }
    },
    permissions: ["MANAGE_MESSAGES"],
}