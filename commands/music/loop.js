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
            serverConfig.loopQueue = false;
            writeToJson()

            if (serverConfig.loopTrack) { message.reply("Now looping **track**") }
            else { message.reply("Disabled looping") }
            return
        }

        if (arguments[0] == "queue") {
            serverConfig.loopQueue = !serverConfig.loopQueue || true;
            serverConfig.loopTrack = false;
            writeToJson()
            
            if (serverConfig.loopQueue) { message.reply("Now looping **queue**") }
            else { message.reply("Disabled looping") }
            return
        }
    },
    permissions: [],
    requiredRoles: ["Security"],
}