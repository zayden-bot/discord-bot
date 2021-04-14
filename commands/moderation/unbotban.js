const blacklist = require("../../blacklist");

module.exports = {
    commands: ["unbotban", "removebotban"],
    expectedArgs: "<user>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const member = message.mentions.members.first()

        if (!member) {
            message.reply("Please mention a valid user.")
            return
        }

        if (blacklist.isBlacklisted(member.user.id)) {
            blacklist.removeBlacklist(member.user.id);
            message.reply("You have succesfully removed the blacklist from the user!");
        } else {
            message.reply("The user is not blacklisted!");
        }
    },
    requiredRoles: ["Security"],
}