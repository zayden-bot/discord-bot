const ytdl = require("ytdl-core")
const common = require("./common")
const serverConfig = require("./serverConfigs/745662812335898806.json")

let dispatcher;

module.exports = {
    getDispatcher: function() {
        return dispatcher
    },

    getSongTitle: async function(link) {
        const info = await ytdl.getInfo(link);
        return info.videoDetails.title
    },
    
    play: function(message, connection) {

        dispatcher = connection.play(ytdl(serverConfig.musicQueue[serverConfig.trackPosition], { filter: 'audioonly' }))
        
        module.exports.getSongTitle(serverConfig.musicQueue[serverConfig.trackPosition])
        .then(songTitle => {
            console.log(`Now Playing: ${songTitle}`)
            message.channel.send(`Now Playing: ${songTitle}`)
        })

        dispatcher.on("finish", () => { // Track Pos: 0; Length = 1
            if (serverConfig.loopQueue) {
                serverConfig.musicQueue.push(serverConfig.musicQueue.shift()) // Move first item to end of array
            }

            if (!serverConfig.loopTrack) {
                serverConfig.trackPosition += 1
            }

            if (serverConfig.musicQueue) { module.exports.play(message, connection) }
            else { module.exports.disconnect(connection) }
        })
    },

    skip: function() {
        if (dispatcher) { dispatcher.end() }
    },

    back: function() {
        serverConfig.trackPosition -= 2;
        common.writeToServerConfig("745662812335898806")
        if (dispatcher) { dispatcher.end() }
    },

    clear: function() {
        serverConfig.musicQueue = []
        common.writeToServerConfig("745662812335898806")

    },

    jump: function(trackPosition) {
        trackPosition -= 2;
        serverConfig.trackPosition = trackPosition
        common.writeToServerConfig("745662812335898806")
        if (dispatcher) { dispatcher.end() }
    },

    pause: function() {
        if (dispatcher) { dispatcher.pause(true); }
    },

    resume: function() {
        if (dispatcher) { dispatcher.resume() }
    },

    remove: function(trackPosition) {
        serverConfig.musicQueue = serverConfig.musicQueue.filter((value, index, arr) => {
            return index != trackPosition - 1
        })
        common.writeToServerConfig("745662812335898806")
    },

    removeRange: function(start, end) {
        serverConfig.musicQueue = serverConfig.musicQueue.filter((value, index, arr) => {
            return start - 1 > index && index < end
        })
        console.log(serverConfig.musicQueue)
        common.writeToServerConfig("745662812335898806")
    },

    disconnect: function(connection) {
        serverConfig.musicQueue = []
        serverConfig.trackPosition = 0
        common.writeToServerConfig("745662812335898806")

        serverConfig.loopTrack = false;
        serverConfig.loopQueue = false;

        if (dispatcher) { dispatcher.destroy(); }

        connection.disconnect();


    }

}