import Discord from "discord.js"

module.exports = {
    commands: ["newyear", "ny"],
    callback: async (message: Discord.Message) => {
        if (message.author.id == "211486447369322506") {
            message.channel.send(`Thank you Master ${message.author.username} for letting me survive to 2023 <:pandahappy:788512955641495592>`)
        } else {
            await message.reply("Happy New Year!")
        }
    },
}
