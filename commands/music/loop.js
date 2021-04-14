const fs = require("fs")
const serverConfig = require("../../serverConfigs/745662812335898806.json")

function writeToJson() {
    fs.writeFile("./serverConfigs/745662812335898806.json", JSON.stringify(serverConfig, null, 4), function writeJSON(err) {
        if (err) { return console.log(err); }
    });
}

module.exports = {
    commands: ["loop"],
    expectedArgs: "<track/song/queue>",
    permissionError: "Command is currently in development. Limited to staff use only.",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        if (arguments[0] == "track" || arguments[0] == "song") {
            serverConfig.loopTrack = !serverConfig.loopTrack || true;
            serverConfig.loopQueue = !serverConfig.loopQueue || false;
            writeToJson()
            message.reply("Now looping **track**")
            return
        }

        if (arguments[0] == "queue") {
            serverConfig.loopQueue = !serverConfig.loopQueue || true;
            serverConfig.loopTrack = !serverConfig.loopTrack || false;
            writeToJson()
            message.reply("Now looping **queue**")
            return
        }
    },
    permissions: [],
    requiredRoles: ["Security"],
}