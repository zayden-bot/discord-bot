const music = require("../../musicFunctions")
const serverConfig = require("../../serverConfigs/745662812335898806.json")
module.exports = {
    commands: ["remove", "r", "rm", "delete", "del"],
    permissionError: "Command is currently in development. Limited to staff use only.",
    callback: (message, arguments, text) => {
        let trackPosition = arguments[0]
        if (arguments[0] == "last") {
            trackPosition = serverConfig.musicQueue.length
        }

        music.getSongTitle(serverConfig.musicQueue[trackPosition - 1]).then(songTitle => {
            music.remove(trackPosition)
            message.channel.send(`Removed ${songTitle} from queue.`)
        })
    },
    permissions: [],
    requiredRoles: [],
}