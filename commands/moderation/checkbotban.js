const Discord = require("discord.js");
const blacklist = require("../../blacklist");

module.exports = {
    commands: "checkbotban",
    expectedArgs: "<user>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const memberId = arguments[0].match(/\d+/)[0];
        const member = message.guild.members.cache.get(memberId)

        if (!member) {
            message.reply("Please mention a valid user.")
            return
        }

        if (blacklist.isBlacklisted(member.id)) {
            message.reply("The user is blacklisted!");
        } else {
            message.reply("The user is not blacklisted!");
        }
    },
    permissions: ["MANAGE_MESSAGES"],
}