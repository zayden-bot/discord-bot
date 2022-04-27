import Discord from "discord.js"

module.exports = {
    commands: "ping",
    callback: async (message: Discord.Message) => {
        await message.reply("Pong!")
    },
}