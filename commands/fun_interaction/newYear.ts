import Discord from "discord.js"

module.exports = {
    commands: ["newyear", "ny"],
    callback: (message: Discord.Message, args: string[], text: string) => {
        if (message.author.id == "211486447369322506") {
            message.channel.send(`Thank you Master ${message.author.username} for letting me survive to 2022 <:pandahappy:788512955641495592>`)
        } else { message.reply("Happy New Year!") }
    },
}
