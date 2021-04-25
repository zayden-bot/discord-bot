const sql = require("../../sql")

module.exports = {
    commands: ["clearwarnings", "clearwarns"],
    expectedArgs: "<user>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const member = message.mentions.members.first()
        if (!member) {
            message.reply("Please mention a valid member")
            return
        }

        const index = require("../../index")
        let server = index.servers[message.guild.id]

        console.log(server.moderation)

        for (log in server.moderation) {
            if (server.moderation[log].userId == member.id && server.moderation[log].type == "warning") {
                sql.run(`DELETE FROM 'moderation' WHERE caseNumber = '${Number(log)}';`);
                delete server.moderation[log]
            }
        }
        console.log(server.moderation)
    },
    requiredRoles: ["Security"],
}