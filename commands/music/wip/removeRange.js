const music = require("../../../musicFunctions")

module.exports = {
    commands: ["removeRange", "rr"],
    permissionError: "Command is currently in development. Limited to staff use only.",
    disabled: true,
    callback: (message, arguments, text) => {
        start = parseInt(arguments[0])
        end = parseInt(arguments[1])
        music.removeRange(start, end)
        message.channel.send(`Removed ${(end + 1) - start} songs from queue.`)
    },
    permissions: [],
    requiredRoles: [],
}