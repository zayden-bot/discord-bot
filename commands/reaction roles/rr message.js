const reactionRoles = require("../../reactionRoleFuncions")

module.exports = {
    commands: ["rr message", "rrm"],
    expectedArgs: "<message>",
    minArgs: 1,
    maxArgs: 1,
    callback: (message, arguments, text) => {
        reactionRoles.setMessageId(text)
    },
    permissions: ["ADMINISTRATOR"],
}