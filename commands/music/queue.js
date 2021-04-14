const ytdl = require("ytdl-core")

const serverConfig = require("../../serverConfigs/CKConfig.json")

async function getQueue() {
    let queue = ""
    for (let i = 0; i < serverConfig.musicQueue.length; i++) {
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
        getQueue().then( queue => message.channel.send(`**Music Queue**\n\n${queue}`) );

        
    },
    permissions: [],
    requiredRoles: ["Security"],
}