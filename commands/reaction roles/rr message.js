const reactionRoles = require("../../reactionRoles")

module.exports = {
    commands: ["rr message", "rrm"],
    expectedArgs: "<message>",
    minArgs: 1,
    maxArgs: 1,
    callback: (message, arguments, text) => {
        reactionRoles.getMessage(arguments[0])
    },
    permissions: [],
    requiredRoles: ["Security"],
}