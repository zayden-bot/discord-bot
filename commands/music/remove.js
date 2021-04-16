const music = require("../../musicFunctions")

module.exports = {
    commands: ["remove", "r", "rm", "delete", "del"],
    permissionError: "Command is currently in development. Limited to staff use only.",
    expectedArgs: "<track position>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const queue = music.servers[message.guild.id].queue
        
        let trackPosition = arguments[0]
        if (trackPosition > queue.currentQueue.length) {
            message.reply("Track position out of queue length")
            return
        }

        if (arguments[0] == "last") {
            trackPosition = queue.currentQueue.length
        }
    
        const song = queue.currentQueue[trackPosition - 1].title
        music.remove(message, trackPosition)
        message.channel.send(`Removed ${song} from queue.`)
    },
    permissions: [],
    requiredRoles: [],
}