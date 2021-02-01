const Discord = require("discord.js")
const sql = require("./sql");

let channelId = "787713329472471050";
let messageId = "805707849246179339";

let reactionMessages = [];
let checkReactionMessage = [787713329472471000, 805707849246179300]

module.exports = {
    init: function () {
        sql.each("SELECT * FROM `reactionRoleMessages`", (row) => {
            reactionMessages.push([Number(row.channelId), Number(row.messageId), String(row.emoji), Number(row.roleId)]);
            // checkReactionMessage.push([Number(row.channelId), Number(row.messageId), String(row.emoji)]);
        })
        console.log(`Loaded ${reactionMessages.length} reaction messages!`);
    },

    getChannel: function(channel) { // Get Channel ID
        channelId = channel.match(/\d+/);
        // console.log(`ChannelID: ${channelId}`);
    },

    getMessage: function(message) { // Get Message ID
        messageId = message.match(/\d+/);
    },

    addNormalReaction: function(message, emoji, role) {
        const roleId = role.match(/\d+/);
        message.client.channels.fetch(channelId).then(channel => {
            channel.messages.fetch(messageId).then(message => {
                if (checkReactionMessage.includes([Number(channelId), Number(message.id), String(emoji)])) {
                    message.reply("Reaction already exists.")
                } else {
                    // console.log(message)
                    message.react(emoji)
                    sql.run(`INSERT INTO 'reactionRoleMessages' ('channelId', 'messageId', 'emoji', 'roleId') VALUES ('${Number(channel.id)}', '${Number(message.id)}', '${String(emoji)}', '${Number(roleId)}');`)
                    reactionMessages.push([Number(channel.id), Number(message.id), String(emoji), Number(roleId)]);
                    checkReactionMessage.push([Number(channel.id), Number(message.id), String(emoji)]);
                }
            })
        })
    },

    addNormalReactionRole: function(client) {
        client.on("messageReactionAdd", (reaction, user) => {
            const test = [Number(reaction.message.channel.id), Number(reaction.message.id)]
            // console.log(reactionMessages)
            // console.log(checkReactionMessage)
            // console.log(test)

            for (let i = 0; i < checkReactionMessage.length; i++) {
                if (test == checkReactionMessage[i]) {
                    console.log("TRUE")
                } else {
                    console.log("FALSE")
                }
            }

            if (checkReactionMessage.includes([Number(reaction.message.channel.id), Number(reaction.message.id), String(reaction._emoji)]) && user.id !== "787490197943091211") {
                console.log("Reaction Found.")
                const emoji = reaction._emoji.name
                const { guild } = reaction.message
                const reactionMessagesIndex = checkReactionMessage.indexOf([Number(reaction.message.channel.id), Number(reaction.message.id), String(reaction._emoji)])
                const roleId = reactionMessages[reactionMessagesIndex][3]

                const role = guild.roles.cache.get(roleId)
                const member = guild.members.cache.find(member => member.id === user.id)
                
                member.roles.add(role)
            }
        })
    }
}