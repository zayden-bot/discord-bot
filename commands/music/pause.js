const music = require("../../musicFunctions")

module.exports = {
    commands: ["pause"],
    maxArgs: 0,
    permissionError: "Command is currently in development. Limited to staff use only.",
    callback: (message, arguments, text) => {
        music.pause()
    },
}