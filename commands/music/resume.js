const music = require("../../musicFunctions")

module.exports = {
    commands: ["resume"],
    permissionError: "Command is currently in development. Limited to staff use only.",
    callback: (message, arguments, text) => {

        if (!message.guild.voice.connection.dispatcher) {
            message.reply("No music playing.")
            return
        }

        music.resume()

    },
    permissions: [],
    requiredRoles: [],
}