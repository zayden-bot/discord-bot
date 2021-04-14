const reactionRoles = require("../../reactionRoles")

module.exports = {
    commands: ["rr add", "rradd"],
    expectedArgs: "<emoji> <role>",
    minArgs: 2,
    maxArgs: 2,
    callback: (message, arguments, text) => {
        reactionRoles.addNormalReaction(message, arguments[0], arguments[1])
    },
    permissions: ["ADMINISTRATOR"],
}