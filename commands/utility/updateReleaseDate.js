const fs = require('fs');
const config = require("../../serverConfigs/CKConfig.json")
const fileName = "CKConfig.json"

module.exports = {
    commands: ["UpdateReleaseDate", "UpdateUpdate"],
    expectedArgs: "<patreon day> <patreon month> <patreon year> <steam day> <steam month> <steam year>\nExample: 12 Feb 2021 5 Mar 2021",
    permissionError: "",
    minArgs: 6,
    maxArgs: 6,
    callback: (message, arguments, text) => {

        config.patreonReleaseDate = `${arguments[0]} ${arguments[1]}, ${arguments[2]} 19:00:00`;
        config.SteamReleaseDate = `${arguments[3]} ${arguments[4]}, ${arguments[5]} 19:00:00`;

        fs.writeFile(fileName, JSON.stringify(file, null, 4), function writeJSON(err) {
            if (err) return console.log(err);
          });

        message.reply(`Release Dates Updated`)
    },
    permissions: [],
    requiredRoles: ["Security"],
}