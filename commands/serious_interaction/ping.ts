import Discord from "discord.js"

module.exports = {
    commands: "ping",
    minArgs: 0,
    maxArgs: 0,
    callback: async (message: Discord.Message) => {
        await message.reply("Pong!")
    },
}