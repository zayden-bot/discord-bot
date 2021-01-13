const Discord = require("discord.js");
const blacklist = require("../../blacklist");

module.exports = {
    commands: "checkbotban",
    expectedArgs: "<user>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const member = message.mentions.members.first()
        if (blacklist.isBlacklisted(member.user.id)) {
            message.reply("The user is blacklisted!");
        } else {
            message.reply("The user is not blacklisted!");
        }
    },
    requiredRoles: ["Security"],
}