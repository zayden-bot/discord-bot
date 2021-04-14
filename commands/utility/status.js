const fs = require('fs');
const fileName = "botConfig.json";
const file = require("../../Configs/botConfig.json");

module.exports = {
    commands: "status",
    expectedArgs: "<status>",
    minArgs: 1,
    callback: (message, arguments, text) => {

        message.client.user.setPresence({
            activity: {
                name: text,
                type: 0,
            },
        })

        message.reply(`Status Changed to: Playing ${text}`)
    },
    permissions: [],
    requiredRoles: ["Security"],
}
