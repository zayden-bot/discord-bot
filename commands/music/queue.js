const music = require("../../musicFunctions")
const serverConfig = require("../../serverConfigs/745662812335898806.json")

async function getQueue() {
    if (serverConfig.trackPosition + 1 >= serverConfig.musicQueue.length) {
        return "End of queue. Use `!play` to queue some music up."
    }

    let queue = ""
    for (let i = serverConfig.trackPosition + 1; i < serverConfig.musicQueue.length; i++) {
        console.log(serverConfig.musicQueue[i])
        const songTitle = await music.getSongTitle(serverConfig.musicQueue[i]);
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
            message.channel.send("End of queue. Use `!play` to queue some music up.")
            return;
        }

        let nowPlaying = "None"
        music.getSongTitle(serverConfig.musicQueue[serverConfig.trackPosition]).then(songTitle => {
            nowPlaying = songTitle

            getQueue().then(queue => {
                message.channel.send(`**Music Queue**\nTrack Position: ${serverConfig.trackPosition+1}\nNow Playing: ${nowPlaying}\n\n${queue}`) 
            });
            
        })
    },
    permissions: [],
    requiredRoles: [],
}