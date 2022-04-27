import Discord from "discord.js";

module.exports = {
    commands: ["cc", "clearchannel", "purgeall"],
    callback: async (message: Discord.Message) => {
        const results = await message.channel.messages.fetch()
        if (message.channel.type != "DM") {
            message.channel.bulkDelete(results)
        }
    },
    permissions: ["ADMINISTRATOR"],
}