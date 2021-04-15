const music = require("../../musicFunctions")

module.exports = {
    commands: ["resume"],
    permissionError: "Command is currently in development. Limited to staff use only.",
    callback: (message, arguments, text) => {
        if (music.getDispatcher()) {
            music.resume()
            return
        }

        if (!message.member.voice.channel) {
            message.reply("You have to be connected to a voice channel before you can use this command!")
            return
        }

        if (!message.guild.voice) {
            message.member.voice.channel.join().then(connection => { music.play(message, connection) })
        }

    },
    permissions: [],
    requiredRoles: [],
}