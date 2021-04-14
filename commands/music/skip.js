const music = require("../../musicFunctions")
const serverConfig = require("../../serverConfigs/CKConfig.json")

module.exports = {
    commands: ["skip", "next", "n"],
    permissionError: "Command is currently in development. Limited to staff use only.",
    callback: (message, arguments, text) => {
        music.skip()
        music.getSongTitle(serverConfig.musicQueue[serverConfig.trackPosition]).then(songTitle => message.channel.send(`Now Playing: ${songTitle}`))
    },
    permissions: [],
    requiredRoles: ["Security"],
}