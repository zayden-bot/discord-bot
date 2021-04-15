const music = require("../../musicFunctions")
const serverConfig = require("../../serverConfigs/745662812335898806.json")
module.exports = {
    commands: ["remove", "r", "rm", "delete", "del"],
    permissionError: "Command is currently in development. Limited to staff use only.",
    callback: (message, arguments, text) => {
        track = arguments[0]
        if (arguments[0] == "last") {
            track = serverConfig.musicQueue.length
        }

        music.remove(track)
        music.getSongTitle(serverConfig.musicQueue[track + 1]).then(songTitle => { message.channel.send(`Removed ${songTitle} from queue.`) })
    },
    permissions: [],
    requiredRoles: [],
}