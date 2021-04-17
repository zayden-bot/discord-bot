const music = require("../../musicFunctions")

module.exports = {
    commands: ["disconnect", "dc", "leave", "reset", "fuckoff"],
    maxArgs: 0,
    callback: (message, arguments, text) => {
        if (!message.guild.voice || !message.guild.voice.channelID) {
            message.reply("Not in voice channel. Use `!play` to queue some music up.")
            return
        }
        const connection = message.guild.voice.connection
        music.disconnect(message, connection)
        message.channel.send("Disconnecting...")
    },
}