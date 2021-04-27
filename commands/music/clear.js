const music = require("../../musicFunctions")

module.exports = {
    commands: ["clear"],
    maxArgs: 0,
    callback: (message, arguments, text) => {
        music.clear(message.guild)
        message.channel.send("Queue cleared.")
    },
}