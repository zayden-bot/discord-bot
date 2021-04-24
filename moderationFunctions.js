const sql = require("./sql")


module.exports = {
    
    init: () => {
        const { servers } = require("./index")
        sql.each("SELECT * FROM `moderation`", (row) => {
            servers[row.guildId].moderation[row.caseNumber] = {
                "userId": row.userId,
                "type": row.type,
                "moderator": row.moderator,
                "reason": row.reason
            };
        })
        console.log(`Loaded ${Object.keys(servers).length} servers!`);
    },

    addLog: (guild, user, type, moderator, reason) => {
        const { servers } = require("./index")
        let server = servers[guild.id]

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
        const { servers } = require("./index")
        let server = servers[guild.id]
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
        const { servers } = require("./index")
        let server = servers[guild.id]
        let logs = {}

        for (log in server.moderation) {
            if (server.moderation[log].userId == user.id) {
                logs[log] = server.moderation[log]
            }
        }
        return logs
    }
}