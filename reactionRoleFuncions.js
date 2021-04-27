const sql = require("./sql");

let channelId;
let messageId;

let reactionMessages = [];
let checkReactionMessages = []

module.exports = {
    init: function () {
        const { servers } = require("./index")
        sql.each("SELECT * FROM `reactionRoles`", (row) => {
            servers[row.guildId].reactionRoles[row.id] = {
                "channelId": row.channelId,
                "messageId": row.messageId,
                "emoji": row.emoji,
                "roleId": row.roleId
            }
        })
    },

    setChannelId: function(channel) { // Get Channel ID
        channelId = channel.match(/\d+/)[0];
    },

    getChannelId: () => {
        return channelId
    },

    setMessageId: function(message) { // Get Message ID
        messageId = message.match(/\d+/)[0];
    },

    getMessageId: () => {
        return messageId
    },

    // Add the reaction to message + database
    addNormalReaction: function(client, guildId, emoji, role) {
        const roleId = role.match(/\d+/);
        client.channels.fetch(channelId).then(channel => {
            channel.messages.fetch(messageId).then(msg => {
                const { servers } = require("./index");
                const { reactionRoles } = servers[guildId];

                const id = Object.keys(reactionRoles).length
                sql.run(`INSERT INTO 'reactionRoles' ('id', 'guildId', 'channelId', 'messageId', 'emoji', 'roleId') VALUES ('${id}', '${guildId}', '${channelId}', '${messageId}', '${emoji}', '${roleId}');`)
                msg.react(emoji)
                
                reactionRoles[id] = {
                    "channelId": channelId,
                    "messageId": messageId,
                    "emoji": emoji,
                    "roleId": roleId
                }
            })
        })
    },

    addNormalReactionRole: client => { // Watch for reaction and grant role
        client.on("messageReactionAdd", (reaction, user) => {
            const { servers } = require("./index");
            const { guild } = reaction.message
            const { reactionRoles } = servers[guild.id];

            for (reactionRoleId in reactionRoles) {
                if (reaction.message.id == reactionRoles[reactionRoleId].messageId && reaction.message.channel.id == reactionRoles[reactionRoleId].channelId && reaction._emoji.name == reactionRoles[reactionRoleId].emoji && user.id !== "787490197943091211") {
                    let roleId = reactionRoles[reactionRoleId].roleId

                    if (typeof(roleId) == "string") { roleId = [roleId] }
                    const role = guild.roles.cache.get(roleId[0])

                    const member = guild.members.cache.find(member => member.id == user.id)
                    
                    member.roles.add(role)
                    break
                }
            }
        })
    },

    removeNormalReactionRole: function(client) { // Remove reaction Role
        client.on("messageReactionRemove", (reaction, user) => {
            const { servers } = require("./index");
            const { guild } = reaction.message
            const { reactionRoles } = servers[guild.id];

            for (reactionRoleId in reactionRoles) {
                if (reaction.message.id == reactionRoles[reactionRoleId].messageId && reaction.message.channel.id == reactionRoles[reactionRoleId].channelId && reaction._emoji.name == reactionRoles[reactionRoleId].emoji && user.id !== "787490197943091211") {
                    let roleId = reactionRoles[reactionRoleId].roleId
                    
                    if (typeof(roleId) == "string") { roleId = [roleId] }
                    const role = guild.roles.cache.get(roleId[0])

                    const member = guild.members.cache.find(member => member.id == user.id)
                    
                    member.roles.remove(role)
                    break
                }
            }
        })
    }
}