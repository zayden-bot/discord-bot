const music = require("../../musicFunctions")
const serverConfig = require("../../serverConfigs/745662812335898806.json")

module.exports = {
    commands: ["disconnect", "dc", "leave", "reset", "fuckoff"],
    maxArgs: 0,
    callback: (message, arguments, text) => {
        if (!message.guild.voice || !message.guild.voice.channelID) {
            message.reply("Not in voice channel. Use `!play` to queue some music up.")
            return
        }
        const connection = message.guild.voice.connection
        music.disconnect(connection)
        message.channel.send("Disconnecting...")
    },
}