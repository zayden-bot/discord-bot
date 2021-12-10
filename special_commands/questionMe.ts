import Discord from "discord.js"

module.exports = function(message: Discord.Message) {
    if (!message.client.user) { return; }

    const blacklist = require("../blacklist");
    if (message.mentions.users.size && message.mentions.users.first()?.id == message.client.user.id && message.content.slice(-1) == "?" && !blacklist.isBlacklisted(message.author.id)) {
        if (Math.floor(Math.random() * 2)) {  message.reply("Yes"); }
        else { message.reply("No"); }
    }
}