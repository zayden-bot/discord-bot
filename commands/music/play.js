const music = require("../../musicFunctions")

let Queue = music.Queue

module.exports = {
    commands: ["play", "p"],
    expectedArgs: "<music>",
    minArgs: 1,
    callback: async (message, arguments, text) => {
        if (!message.member.voice.channel) {
            message.reply("You have to be connected to a voice channel before you can use this command!")
            return
        }

        // Create guild config
        try { music.servers[message.guild.id].queue }
        catch (error) { music.servers[message.guild.id] = {} }

        if (!music.servers[message.guild.id].queue) {
            music.servers[message.guild.id].queue = new Queue(message.guild.id)
        }

        let queue = music.servers[message.guild.id].queue

        const videoPattern = /^(https?:\/\/)?(www\.)?(m\.)?(youtube\.com|youtu\.?be)\/.+$/gi;
        const playlistPattern = /^.*(list=)([^#\&\?]*).*/gi;

        const url = arguments[0]

        // Adding to queue
        if (playlistPattern.test(url)) { // if youtube playlist
            const urls = await queue.getPlaylist(url);
            message.channel.send(`Added ${urls.length} songs to the Queue.`);
        } else if (videoPattern.test(url)) { // if single link
            const songTitle = await queue.getSong(url)
            message.channel.send(`Added ${songTitle} to the Queue.`)
        } else {
            const songTitle = await queue.getSearch(text)

            // Check if song is found
            if (songTitle) {
                message.channel.send(`Added ${songTitle} to the Queue.`)
            } else {
                message.channel.send("No song found")
                return
            }
        }

        if (!message.guild.voice || !message.guild.voice.channelID && typeof(music.dispatcher) == "undefined") {
            message.member.voice.channel.join().then(connection => { music.play(message, connection) })
        }

    },
    permissions: [],
    requiredRoles: [],
}