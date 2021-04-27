const reactionRoles = require("../../reactionRoleFuncions")

module.exports = {
    commands: ["rr add", "rradd"],
    expectedArgs: "<emoji> <role>",
    minArgs: 2,
    maxArgs: 2,
    callback: (message, arguments, text) => {
        const { servers } = require("../../index")
        const server = servers[message.guild.id]

        const emoji = arguments[0]
        const role = arguments[1]

        for (reactionRoleId in server.reactionRoles) {
            if (reactionRoles.getChannelId() == server.reactionRoles[reactionRoleId].channelId && reactionRoles.getMessageId() == server.reactionRoles[reactionRoleId].messageId && emoji == server.reactionRoles[reactionRoleId].emoji) {
                message.reply("Reaction already exists for this message.")
                return
            }
        }

        reactionRoles.addNormalReaction(message.client, message.guild.id, emoji, role)
    },
    permissions: ["ADMINISTRATOR"],
}