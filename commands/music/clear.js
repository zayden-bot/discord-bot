const music = require("../../musicFunctions")

module.exports = {
    commands: ["clear"],
    permissionError: "Command is currently in development. Limited to staff use only.",
    callback: (message, arguments, text) => {
        music.clear()
        message.channel.send("Queue cleared.")
    },
    permissions: [],
    requiredRoles: ["Security"],
}