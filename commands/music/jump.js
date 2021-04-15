const music = require("../../musicFunctions")
const serverConfig = require("../../serverConfigs/745662812335898806.json")

module.exports = {
    commands: ["jump", "j", "goto"],
    expectedArgs: "<track position>",
    minArgs: 1,
    maxArgs: 1,
    callback: (message, arguments, text) => {
        trackPosition = Number(arguments[0])
        if (isNaN(trackPosition)) {
            message.reply("Please enter a valid track position")
        }
        if (trackPosition > serverConfig.musicQueue.length) {
            message.reply("Track not in queue.")
        }
        music.jump(trackPosition)
    },
}