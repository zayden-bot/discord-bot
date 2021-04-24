const reactionRoles = require("../../reactionRoleFuncions")

module.exports = {
    commands: ["rr channel", "rrc"],
    expectedArgs: "<channel>",
    minArgs: 1,
    maxArgs: 1,
    callback: (message, arguments, text) => {
        reactionRoles.getChannel(arguments[0])
    },
    permissions: ["ADMINISTRATOR"],
}