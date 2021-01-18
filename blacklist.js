const config = require("./config.json");
const discord = require("discord.js");
const sql = require("./sql");
var blacklistedUsers = [];

module.exports = {
    init: function () {
        blacklistedUsers = [];
        sql.each("SELECT * FROM `blacklist`", (row) => {
            blacklistedUsers.push(Number(row.id));
        })
        console.log(`Loaded ${blacklistedUsers.length} blacklisted members!`);
    },
    isBlacklisted: function(id) {
        if (id) {
            if (blacklistedUsers.includes(Number(id))) {
                return true
            } else {
                return false
            }
        } else {
            console.error("Expected int at argument 1 of isBlacklisted");
        }
    },
    blacklist: function(id) {
        if (id) {
            if (blacklistedUsers.includes(Number(id))) {
                return false // "User is already blacklisted"
            } else {
                blacklistedUsers.push(Number(id));

                sql.run(`INSERT INTO 'blacklist' ('id') VALUES ('${Number(id)}');`)
            }
        } else {
            console.error("Expected int at argument 1 of blacklist");
        }
    },
    removeBlacklist: function(id) {
        if (id) {
            if (blacklistedUsers.includes(Number(id))) {
                var index = blacklistedUsers.indexOf(Number(id));
                if (index > -1) {
                    blacklistedUsers.splice(index, 1);

                    sql.run(`DELETE FROM 'blacklist' WHERE id = '${Number(id)}';`);
                }
            } else {
                return false // "User is not blacklisted"
            }
        } else {
            console.error("Expected int at argument 1 of removeBlacklist");
        }
    },
    isProtectedUser: function(id) {
        if (id) {
            if (config.protectedUsers.includes(String(id))) {
                return true
            } else {
                return false
            }
        } else {
            console.error("Expected int at argument 1 of isProtectedUser");
        }
    }
}