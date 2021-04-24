const { servers } = require("../../index")
const music = require("../../musicFunctions")

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

        const queue = servers[message.guild.id].queue
        if (trackPosition > queue.currentQueue.length) {
            message.reply("Track not in queue.")
            return
        }
        music.jump(message, trackPosition)
    },
}