const Discord = require("discord.js")
const sql = require("./sql");

let channelId = "787713329472471050";
let messageId = "805707849246179339";

let reactionMessages = [];
let checkReactionMessages = []

module.exports = {
    init: function () {
        sql.each("SELECT * FROM `reactionRoleMessages`", (row) => {
            reactionMessages.push([row.channelId, row.messageId, row.emoji, row.roleId]);
            checkReactionMessages.push([row.channelId, row.messageId, row.emoji]);
        })
        console.log(`Loaded ${reactionMessages.length} reaction messages!`);
    },

    getChannel: function(channel) { // Get Channel ID
        channelId = channel.match(/\d+/);
    },

    getMessage: function(message) { // Get Message ID
        messageId = message.match(/\d+/);
    },

    addNormalReaction: function(message, emoji, role) { // Add the reaction to message + database
        const roleId = role.match(/\d+/);
        message.client.channels.fetch(channelId).then(channel => {
            channel.messages.fetch(messageId).then(message => {
                if (checkReactionMessages.includes([channelId, message.id, emoji])) {
                    message.reply("Reaction already exists.")
                } else {
                    message.react(emoji)
                    sql.run(`INSERT INTO 'reactionRoleMessages' ('channelId', 'messageId', 'emoji', 'roleId') VALUES ('${channel.id}', '${message.id}', '${emoji}', '${roleId}');`)
                    reactionMessages.push([channel.id, message.id, emoji, roleId]);
                    checkReactionMessages.push([channel.id, message.id, emoji]);
                }
            })
        })
    },

    addNormalReactionRole: function(client) { // Watch for reaction and grant role
        client.on("messageReactionAdd", (reaction, user) => {
            let check = false
            let index;

            for (let i = 0; i < checkReactionMessages.length; i++) {
                if (checkReactionMessages[i][0] == reaction.message.channel.id && checkReactionMessages[i][1] == reaction.message.id && checkReactionMessages[i][2] == reaction._emoji.name) {
                    check = true
                    index = i
                }
            }

            if (check && user.id !== "787490197943091211") {
                const { guild } = reaction.message
                const roleId = reactionMessages[index][3]

                const role = guild.roles.cache.get(roleId)
                const member = guild.members.cache.find(member => member.id === user.id)
                
                member.roles.add(role)
            }
        })
    },

    removeNormalReactionRole: function(client) { // Remove reaction Role
        client.on("messageReactionRemove", (reaction, user) => {
            let check = false
            let index;

            for (let i = 0; i < checkReactionMessages.length; i++) {
                if (checkReactionMessages[i][0] == reaction.message.channel.id && checkReactionMessages[i][1] == reaction.message.id && checkReactionMessages[i][2] == reaction._emoji.name) {
                    check = true
                    index = i
                }
            }

            if (check && user.id !== "787490197943091211") {
                const { guild } = reaction.message
                const roleId = reactionMessages[index][3]

                const role = guild.roles.cache.get(roleId)
                const member = guild.members.cache.find(member => member.id === user.id)
                
                member.roles.remove(role)
            }
        })
    }
}