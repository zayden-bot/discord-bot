const YouTubeAPI = require("simple-youtube-api")

const ytdl = require("ytdl-core")
const common = require("./common")
const botConfig = require("./configs/botConfig.json")
const serverConfig = require("./serverConfigs/745662812335898806.json")

const youtube = new YouTubeAPI(botConfig.youtubeAPIKey)

let dispatcher;
let servers = {}

class Queue {
    constructor(guild) {
        this.guild = guild
        this.nowPlaying;
        this.currentQueue = []
        this.previousQueue = []
        this.trackPosition = 0
        this.loopQueue = false
        this.loopTrack = false
    }

    async getPlaylist(url) {
        const results = await youtube.getPlaylist(url, { part: "snippet" });
        let videos = await results.getVideos(50, { part: "snippet" });
        videos = videos.filter((video) => video.title != "Private video" && video.title != "Deleted video")
        
        for (let video of videos) {
            this.addSong(video.url, video.title)
        }
        return videos
    }

    async getSong(url) {
        const song = await youtube.getVideo(url)
        this.addSong(url, song.title)
        return song.title
    }

    async getSearch(search) {
        const results = await youtube.searchVideos(search, 1, { part: "snippet" });
        const song = results[0]
        try { song.url }
        catch { return null }

        this.addSong(song.url, song.title)
        return song.title
    }

    addSong(url, title) {
        let song = new Song(url, title)
        this.currentQueue.push(song)
    }

    clearQueue() {
        this.previousQueue = []
        this.currentQueue = []
    }
}

class Song {
    constructor (url, title) {
        this.url = url;
        this.title = title;
    }
}

module.exports = {
    Queue: Queue,

    Song: Song,
    
    play: function(message, connection) {
        let previousMessage = null
        let queue = servers[message.guild.id].queue

        if (!queue.loopTrack) {
            queue.nowPlaying = queue.currentQueue.shift()
        }

        dispatcher = connection.play(ytdl(queue.nowPlaying.url, { filter: 'audioonly' }))

        console.log(`Now Playing: ${queue.nowPlaying.title}`)

        message.channel.fetchMessage(previousMessage).then(async msg => {
            await message.channel.send(`Now Playing: ${queue.nowPlaying.title}`).then(msg => previousMessage = msg.id)
            if (msg) { msg.delete() }
        })

        dispatcher.on("finish", () => {
            

            if (queue.loopQueue && typeof(queue.currentQueue[0]) == "undefined") {
                queue.currentQueue = [...queue.previousQueue]
                queue.previousQueue = []
            }

            if (queue.currentQueue[0] || queue.loopTrack) { module.exports.play(message, connection) }
            else { setTimeout(() => module.exports.disconnect(message, connection), 5*60*1000)  }
        })
    },

    skip: function() {
        if (dispatcher) { dispatcher.end() }
    },

    back: function(message) {
        let queue = servers[message.guild.id].queue

        if (queue.previousQueue[0]) {
            queue.currentQueue.unshift( queue.previousQueue.pop() )
        } else {
            queue.currentQueue.unshift( queue.currentQueue.pop() )
        }

        module.exports.play(message, message.guild.voice.connection)
    },

    clear: function(message) {
        let queue = servers[message.guild.id].queue
        queue.clearQueue()
    },

    jump: function(message, trackPosition) { // Loop track breaks
        let queue = servers[message.guild.id].queue

        queue.previousQueue = queue.previousQueue.concat(queue.currentQueue.splice(0, trackPosition - 1))

        module.exports.play(message, message.guild.voice.connection)
    },

    pause: function() {
        if (dispatcher) { dispatcher.pause(true); }
    },

    resume: function() {
        if (dispatcher) { dispatcher.resume() }
    },

    remove: function(message, trackPosition) {
        let queue = servers[message.guild.id].queue

        queue.currentQueue.splice(trackPosition - 1, 1)
    },

    removeRange: function(start, end) {
        serverConfig.musicQueue = serverConfig.musicQueue.filter((value, index, arr) => {
            return start - 1 > index && index < end
        })
        console.log(serverConfig.musicQueue)
        common.writeToServerConfig("745662812335898806")
    },

    disconnect: function(message, connection) {
        module.exports.clear(message)

        if (dispatcher) { dispatcher.destroy(); }

        connection.disconnect();

    },

    dispatcher: dispatcher,

    servers: servers,

}