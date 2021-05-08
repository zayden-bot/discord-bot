const sql = require("../../sql")

module.exports = {
    commands: ["clearwarnings", "clearwarns"],
    expectedArgs: "<user>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const memberId = arguments[0].match(/\d+/)[0];
        const member = message.guild.members.cache.get(memberId)
        
        if (!member) {
            message.reply("Please mention a valid member")
            return
        }

        const index = require("../../index")
        let server = index.servers[message.guild.id]

        for (log in server.moderation) {
            if (server.moderation[log].userId == member.id && server.moderation[log].type == "warning") {
                sql.run(`DELETE FROM 'moderation' WHERE caseNumber = '${Number(log)}';`);
                delete server.moderation[log]
            }
        }
        message.channel.send(`Cleared ${member.user.username} warnings.`)
    },
    permissions: ["MANAGE_MESSAGES"],
}