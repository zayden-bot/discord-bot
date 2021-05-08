const blacklist = require("../../blacklist");

module.exports = {
    commands: ["unbotban", "removebotban"],
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
            blacklist.removeBlacklist(member.id);
            message.reply("You have succesfully removed the blacklist from the user!");
        } else {
            message.reply("The user is not blacklisted!");
        }
    },
    permissions: ["MANAGE_MESSAGES"],
}