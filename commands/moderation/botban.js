const blacklist = require("../../blacklist");

module.exports = {
    commands: ["botban", "addbotban"],
    expectedArgs: "<user>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const memberId = arguments[0].match(/\d+/)[0];
        const member = message.guild.members.cache.get(memberId)

        if (!member) {
            message.reply("Please mention a valid user.")
            return
        }

        if (blacklist.isProtectedUser(member.id)) {
            message.reply("Nice try you can't botban that user <:pepepointedlaugh:788514455477813320>");
        } else {
            if (blacklist.isBlacklisted(member.id)) {
                message.reply("User is already blacklisted!");
            } else {
                blacklist.blacklist(member.id);
                message.reply("You have succesfully blacklisted the user!");
            }
        }
    },
    permissions: ["MANAGE_MESSAGES"],
}