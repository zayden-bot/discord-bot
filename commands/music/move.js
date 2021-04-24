const { servers } = require("../../index")
const music = require("../../musicFunctions")

module.exports = {
    commands: ["move"],
    expectedArgs: "<track position> <track position>",
    permissionError: "",
    minArgs: 2,
    maxArgs: 2,
    callback: (message, arguments, text) => {
        let queue = servers[message.guild.id].queue
        const song = queue.currentQueue[arguments[0] - 1].title

        queue.currentQueue.splice(arguments[1] - 1, 0, ...queue.currentQueue.splice(arguments[0] - 1, 1)) 

        message.channel.send(`Moved ${song} to position ${arguments[1]}`)
    },
    permissions: [],
    requiredRoles: ["Security"],
}