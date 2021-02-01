const Discord = require("discord.js")
const config = require("./config.json")

let channelId = "787713329472471050";
let messageId = "805656400843505725";

module.exports = {
    getChannel: function(channel) { // Get Channel ID
        channelId = channel.match(/\d+/);
        // console.log(`ChannelID: ${channelId}`);
    },

    getMessage: function(message) { // Get Message ID
        messageId = message.match(/\d+/);
    },

    addNormalReaction: function(message, emoji, role) {
        const roleId = role.match(/\d+/);
        const RRChannel = message.client.channels.fetch(channelId)
        // const RRMessage = RRChannel.messages.cache.fetch(messageId)
        
        // RRMessage.react(emoji)
        
        // console.log(emoji)
        console.log(RRChannel)
        console.log(RRChannel.messages)
    }
}