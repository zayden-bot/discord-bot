const blacklist = require("../blacklist");

module.exports = function(message) {
    if (message.mentions.users.size && message.mentions.users.first().id == message.client.user.id && message.content.slice(-1) == "?" && !blacklist.isBlacklisted(message.author.id)) {
        if (Math.floor(Math.random() * 2)) {  message.reply("Yes"); }
        else { message.reply("No"); }
    }
}