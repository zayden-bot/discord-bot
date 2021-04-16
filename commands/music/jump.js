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
            return
        }

        const queue = music.servers[message.guild.id].queue
        if (trackPosition > queue.currentQueue.length) {
            message.reply("Track not in queue.")
            return
        }
        music.jump(message, trackPosition)
    },
}