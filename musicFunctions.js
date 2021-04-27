const Discord = require("discord.js")
const YouTubeAPI = require("simple-youtube-api")

const ytdl = require("ytdl-core")
const botConfig = require("./configs/botConfig.json")

const youtube = new YouTubeAPI(botConfig.youtubeAPIKey)

let dispatcher;

class Queue {
    constructor(guild) {
        this.guild = guild
        this.nowPlaying = null;
        this.nowPlayingMessage = null;
        this.currentQueue = []
        this.previousQueue = []
        this.trackPosition = 0
        this.loopQueue = false
        this.loopTrack = false
    }

    async getPlaylist(url, user) {
        const results = await youtube.getPlaylist(url, { part: "snippet" });
        let videos = await results.getVideos(50, { part: "snippet" });
        videos = videos.filter((video) => video.title != "Private video" && video.title != "Deleted video")
        
        for (let video of videos) {
            this.addSong(video.url, video.title, user)
        }
        return videos
    }

    async getSong(url, user) {
        const song = await youtube.getVideo(url)
        this.addSong(url, song.title, user)
        return song.title
    }

    async getSearch(search, user) {
        const results = await youtube.searchVideos(search, 1, { part: "snippet" });
        const song = results[0]
        try { song.url }
        catch { return null }

        this.addSong(song.url, song.title, user)
        return song.title
    }

    addSong(url, title, user) {
        let song = new Song(url, title, user)
        this.currentQueue.push(song)
    }

    clearQueue() {
        this.nowPlaying = null
        this.previousQueue = []
        this.currentQueue = []
    }
}

class Song {
    constructor (url, title, user) {
        this.url = url;
        this.title = title;
        this.user = user
    }
}

module.exports = {
    
    Queue: Queue,

    play: function(message, connection) {
        const { servers } = require("./index")
        let queue = servers[message.guild.id].queue

        if (!queue.loopTrack) {
            queue.nowPlaying = queue.currentQueue.shift()
        }

        dispatcher = connection.play(ytdl(queue.nowPlaying.url, { filter: 'audioonly' }))

        const embed = new Discord.MessageEmbed()
        .setTitle("Now playing")
        .setDescription(`[${queue.nowPlaying.title}](${queue.nowPlaying.url}) [<@${queue.nowPlaying.user.id}>]`)

        try { queue.nowPlayingMessage.edit(embed) }
        catch { message.channel.send(embed).then(msg => queue.nowPlayingMessage = msg ) }

        dispatcher.on("finish", () => {
            queue.previousQueue.push(queue.nowPlaying)
            queue.nowPlaying = null

            if (queue.loopQueue && typeof(queue.currentQueue[0]) == "undefined") {
                queue.currentQueue = [...queue.previousQueue]
                queue.previousQueue = []
            }

            if (queue.currentQueue[0] || queue.loopTrack) { module.exports.play(message, connection) }
        })
    },

    skip: function() {
        if (dispatcher) { dispatcher.end() }
    },

    back: function(message) {
        let queue = index.servers[message.guild.id].queue

        if (queue.previousQueue[0]) {
            queue.currentQueue.unshift( queue.previousQueue.pop() )
        } else {
            queue.currentQueue.unshift( queue.currentQueue.pop() )
        }

        module.exports.play(message, message.guild.voice.connection)
    },

    clear: function(guild) {
        const { servers } = require("./index")
        const queue = servers[guild.id].queue
        
        queue.clearQueue()
    },

    jump: function(message, trackPosition) {
        const { servers } = require("./index")
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
        const { servers } = require("./index")
        let queue = servers[message.guild.id].queue

        queue.currentQueue.splice(trackPosition - 1, 1)
    },

    disconnect: function(guild) {
        const { servers } = require("./index")
        const queue = servers[guild.id].queue
        queue.clearQueue()

        guild.voice.connection.disconnect()

    },

    dispatcher: dispatcher,
}