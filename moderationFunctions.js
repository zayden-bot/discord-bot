const sql = require("./sql")
const index = require("./index")

module.exports = {
    
    init: () => {
        sql.each("SELECT * FROM `moderation`", (row) => {
            index.getServers()[row.guildId].moderation[row.caseNumber] = {
                "userId": row.userId,
                "type": row.type,
                "moderator": row.moderator,
                "reason": row.reason
            };
        })
        console.log(`Loaded ${Object.keys(index.getServers()).length} servers!`);
    },

    addLog: (guild, user, type, moderator, reason) => {
        let server = index.servers[guild.id]

        var caseNumber = Object.keys(server.moderation).length

        server.moderation[caseNumber] = {
            "userId": user.id,
            "type": type,
            "moderator": moderator.id,
            "reason": reason
        }

        sql.run(`INSERT INTO 'moderation' ('caseNumber', 'guildId', 'userId', 'type', 'moderator', 'reason') VALUES ('${caseNumber}', '${guild.id}', '${user.id}', '${type}', '${moderator.id}', '${reason}');`)

    },

    getWarnings: (guild, user) => {
        let server = index.servers[guild.id]
        let warnings = {}

        for (log in server.moderation) {
            if (server.moderation[log].userId == user.id && server.moderation[log].type == "warning") {
                warnings[log] = server.moderation[log]
            }
        }
        return warnings
    },

    mute: () => {
        return
    },

    getLogs: (guild, user) => {
        let server = index.servers[guild.id]
        let logs = {}

        for (log in server.moderation) {
            if (server.moderation[log].userId == user.id) {
                logs[log] = server.moderation[log]
            }
        }
        return logs
    }
}