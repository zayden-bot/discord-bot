const common = require("../../common")
const serverConfig = require("../../serverConfigs/745662812335898806.json")

module.exports = {
    commands: ["loop"],
    expectedArgs: "<track/song/queue/off>",
    minArgs: 1,
    maxArgs: 1,
    callback: (message, arguments, text) => {
        if (arguments[0] == "track" || arguments[0] == "song") {
            serverConfig.loopTrack = !serverConfig.loopTrack;
            serverConfig.loopQueue = false;
            common.writeToServerConfig("745662812335898806")

            if (serverConfig.loopTrack) { message.reply("Now looping **track**") }
            else { message.reply("Disabled looping") }
            return
        }

        if (arguments[0] == "queue") {
            serverConfig.loopQueue = !serverConfig.loopQueue;
            serverConfig.loopTrack = false;
            common.writeToServerConfig("745662812335898806")
            
            if (serverConfig.loopQueue) { message.reply("Now looping **queue**") }
            else { message.reply("Disabled looping") }
            return
        }

        if (arguments[0] == "off") {
            serverConfig.loopQueue = false;
            serverConfig.loopTrack = false;
            common.writeToServerConfig("745662812335898806")
            message.reply("Disabled looping")
            return
        }
    },
}