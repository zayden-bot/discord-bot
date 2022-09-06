import Discord from "discord.js";
import {ChannelType} from "discord-api-types/v10"

module.exports = {
    commands: ["cc", "clearchannel", "purgeall"],
    callback: async (message: Discord.Message) => {
        const results = await message.channel.messages.fetch()
        if (message.channel.type != ChannelType.DM) {
            message.channel.bulkDelete(results)
        }
    },
    permissions: ["ADMINISTRATOR"],
}
