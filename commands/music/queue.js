const { servers } = require("../../index")

function getQueue(queue) {
    // if (serverConfig.trackPosition + 1 >= serverConfig.musicQueue.length) {
    //     return "End of queue. Use `!play` to queue some music up."
    // }

    let queueText = ""
    for (let i = 0; i < 21; i++) {
        if (i < queue.currentQueue.length) {
            const song = queue.currentQueue[i]
            queueText += `${i+1}. ${song.title}\n`
        } else {
            break
        }
    }
    return queueText
}

module.exports = {
    commands: ["queue", "q"],
    permissionError: "Command is currently in development. Limited to staff use only.",
    maxArgs: 1,
    callback: (message, arguments, text) => {

        const queue = servers[message.guild.id].queue

        if (!queue.nowPlaying) {
            message.channel.send("End of queue. Use `!play` to queue some music up.")
            return;
        }

        const queueText = getQueue(queue)
        message.channel.send(`**Music Queue**\nNow Playing: ${queue.nowPlaying.title}\n\n${queueText}`) 

    },
}