import Discord from "discord.js";
import {Server} from "../../../models/server";

module.exports = {
    commands: ["support_ids"],
    maxArgs: 0,
    callback: async (message: Discord.Message) => {
        const guild = message.guild
        if (!guild) {
            return;
        }

        const server = await Server.findOne({id: guild.id}).exec()

        const ids = []
        for (const id in server.supportAnswers) {
            ids.push(id)
        }

        if (ids.length == 0) {
            await message.reply("No support ids for this server")
            return
        }

        await message.reply(`\`\`\`${ids.sort().join("\n")}\`\`\``)
    },
    requiredRoles: ["Support Team"]
}