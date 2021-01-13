const Discord = require("discord.js");
const blacklist = require("../../blacklist");

module.exports = {
    commands: ["botban", "addbotban"],
    expectedArgs: "<user>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const member = message.mentions.members.first()
        if (blacklist.isBlacklisted(member.user.id)) {
            message.reply("User is already blacklisted!");
        } else {
            blacklist.blacklist(member.user.id);
            message.reply("You have succesfully blacklisted the user!");
        }
    },
    requiredRoles: ["Security"],
}