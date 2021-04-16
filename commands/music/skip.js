const music = require("../../musicFunctions")

module.exports = {
    commands: ["skip", "s", "next", "n"],
    permissionError: "Command is currently in development. Limited to staff use only.",
    callback: (message, arguments, text) => {
        music.skip()
    },
    permissions: [],
    requiredRoles: [],
}