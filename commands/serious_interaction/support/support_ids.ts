import Discord from "discord.js";
import { servers } from "../../../server";

module.exports = {
    commands: ["support_ids"],
    maxArgs: 0,
    callback: async (message: Discord.Message, args: string[], text: string) => {
        const guild = message.guild
        if (!guild) { return; }

        const server = servers[guild.id]

        const ids = []
        for (const id in server.supportAnswers) {
            ids.push(id)
        }

        message.reply(`\`\`\`${ids.join("\n")}\`\`\``)
    },
    requiredRoles: ["Admin"]
}