const { servers } = require("../../index")
const music = require("../../musicFunctions")

module.exports = {
    commands: ["loop"],
    expectedArgs: "<track/song/queue/off>",
    minArgs: 1,
    maxArgs: 1,
    callback: (message, arguments, text) => {
        try { var queue = servers[message.guild.id].queue }
        catch (error) {
            message.reply("Queue up some music first.")
            return;
        }

        if (arguments[0] == "track" || arguments[0] == "song") {
            queue.loopTrack = !queue.loopTrack
            queue.loopQueue = false

            if (queue.loopTrack) { message.reply("Now looping **track**") }
            else { message.reply("Disabled looping") }
            return
        }

        if (arguments[0] == "queue"|| arguments[0] == "q") {
            queue.loopTrack = false
            queue.loopQueue = !queue.loopQueue
            
            if (queue.loopQueue) { message.reply("Now looping **queue**") }
            else { message.reply("Disabled looping") }
            return
        }

        if (arguments[0] == "off") {
            queue.loopTrack = false
            queue.loopQueue = false

            message.reply("Disabled looping")
            return
        }
    },
}