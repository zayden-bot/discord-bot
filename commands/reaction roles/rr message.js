const reactionRoles = require("../../reactionRoleFuncions")

module.exports = {
    commands: ["rr message", "rrm"],
    expectedArgs: "<message>",
    minArgs: 1,
    maxArgs: 1,
    callback: (message, arguments, text) => {
        reactionRoles.getMessage(text)
    },
    permissions: ["ADMINISTRATOR"],
}