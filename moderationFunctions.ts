import Discord from "discord.js"
import { servers } from "./server"


export interface Moderation {
    caseNumber: number;
    guildId: string;
    userId: string;
    type: string;
    moderatorId: string;
    reason: string;
}

  
export function init() {
    const sql = require("./sql")
    sql.each("SELECT * FROM `moderation`", (row: Moderation) => {
        servers[row.guildId].moderation[row.caseNumber] = {
            "userId": row.userId,
            "type": row.type,
            "moderator": row.moderatorId,
            "reason": row.reason
        };
    })
    console.log(`Loaded ${Object.keys(servers).length} servers!`);
}

export function addLog (guild: Discord.Guild, member: Discord.GuildMember, type: string, moderator: Discord.GuildMember, reason: string) {
    let server = servers[guild.id]

    const caseNumber = Object.keys(server.moderation).length

    reason = reason.replace("'", "\'")

    server.moderation[caseNumber] = {
        "userId": member.id,
        "type": type,
        "moderator": moderator.id,
        "reason": reason
    }

    const sql = require("./sql")
    sql.run(`INSERT INTO 'moderation' ('caseNumber', 'guildId', 'userId', 'type', 'moderator', 'reason') VALUES ('${caseNumber}', '${guild.id}', '${member.id}', '${type}', '${moderator.id}', '${reason}');`)
}

//     getWarnings: (guild, member) => {
//         let server = servers[guild.id]
//         let warnings = {}

//         for (log in server.moderation) {
//             if (server.moderation[log].userId == member.id && server.moderation[log].type == "warning") {
//                 warnings[log] = server.moderation[log]
//             }
//         }
//         return warnings
//     },

//     mute: () => {
//         return
//     },

//     getLogs: (guild, member) => {
//         let server = servers[guild.id]
//         let logs = {}

//         for (log in server.moderation) {
//             if (server.moderation[log].userId == member.id) {
//                 logs[log] = server.moderation[log]
//             }
//         }
//         return logs
//     }
