const ytdl = require("ytdl-core")

const serverConfig = require("../../serverConfigs/745662812335898806.json")

async function getQueue() {
    let queue = ""
    for (let i = serverConfig.trackPosition; i < serverConfig.musicQueue.length; i++) {
        const info = await ytdl.getInfo(serverConfig.musicQueue[i]);
        const songTitle = info.videoDetails.title
            queue += `${i+1}. ${songTitle}\n`
    }
    return queue
}

module.exports = {
    commands: ["queue", "q"],
    permissionError: "Command is currently in development. Limited to staff use only.",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        if (serverConfig.musicQueue.length == 0) {
            message.reply("The queue is empty. Use `!play` to queue some music up.")
            return
        }
        getQueue().then( queue => message.channel.send(`**Music Queue**\nTrack Position: ${serverConfig.trackPosition+1}\n\n${queue}`) );

        
    },
    permissions: [],
    requiredRoles: ["Security"],
}