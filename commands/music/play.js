const YouTubeAPI = require("simple-youtube-api")

const music = require("../../musicFunctions")
const common = require("../../common")
const serverConfig = require("../../serverConfigs/745662812335898806.json")
const botConfig = require("../../configs/botConfig.json")

const youtube = new YouTubeAPI(botConfig.youtubeAPIKey)

async function getLinks(link) {
    const results = await youtube.getPlaylist(link, { part: "snippet" });
    const videos = await results.getVideos(25, { part: "snippet" });
    return videos.map(video => video.url)
}

async function getSearch(searchTerm) {
    const result = await youtube.searchVideos(searchTerm, 1, { part: "snippet" });
    return result[0].url
}

module.exports = {
    commands: ["play", "p"],
    expectedArgs: "<music>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        if (!message.member.voice.channel) {
            message.reply("You have to be connected to a voice channel before you can use this command!")
            return
        }

        const videoPattern = /^(https?:\/\/)?(www\.)?(m\.)?(youtube\.com|youtu\.?be)\/.+$/gi;
        const playlistPattern = /^.*(list=)([^#\&\?]*).*/gi;

        // Adding to queue
        if (playlistPattern.test(arguments[0])) { // if youtube playlist
            getLinks(arguments[0]).then(links => {
                for (link of links) {
                    serverConfig.musicQueue.push(link)
                }
                music.getSongTitle(arguments[0]).then(songTitle => message.channel.send(`Added ${links.length} songs to the Queue.`))
            })
        } else if (videoPattern.test(arguments[0])) { // if single link
            serverConfig.musicQueue.push(arguments[0])
            music.getSongTitle(arguments[0]).then(songTitle => message.channel.send(`Added ${songTitle} to the Queue.`))
        } else {
            const url = getSearch(text)
            serverConfig.musicQueue.push(url)
            music.getSongTitle(url).then(songTitle => message.channel.send(`Added ${songTitle} to the Queue.`))
            
        }

        common.writeToServerConfig("745662812335898806")

        if (!message.guild.voice || !message.guild.voice.channelID) {
            message.member.voice.channel.join().then(connection => { music.play(message, connection) })
        }
    },
    permissions: [],
    requiredRoles: [],
}