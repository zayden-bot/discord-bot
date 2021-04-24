const index = require("../../index")
const music = require("../../musicFunctions")

module.exports = {
    commands: ["play", "p"],
    expectedArgs: "<music>",
    minArgs: 1,
    callback: async (message, arguments, text) => {
        let queue = index.servers[message.guild.id].queue

        if (!message.member.voice.channel) {
            message.reply("You have to be connected to a voice channel before you can use this command!")
            return
        }

        const videoPattern = /^(https?:\/\/)?(www\.)?(m\.)?(youtube\.com|youtu\.?be)\/.+$/gi;
        const playlistPattern = /^.*(list=)([^#\&\?]*).*/gi;

        const url = arguments[0]

        // Adding to queue
        if (playlistPattern.test(url)) { // if youtube playlist
            const urls = await queue.getPlaylist(url, message.author);
            message.channel.send(`Added ${urls.length} songs to the Queue.`);
        } else if (videoPattern.test(url)) { // if single link
            const songTitle = await queue.getSong(url, message.author)
            message.channel.send(`Added ${songTitle} to the Queue.`)
        } else {
            const songTitle = await queue.getSearch(text, message.author)

            // Check if song is found
            if (songTitle) {
                message.channel.send(`Added ${songTitle} to the Queue.`)
            } else {
                message.channel.send("No song found")
                return
            }
        }

        if (!guild.voice || message.member.voice.channelID != guild.voice.channelID) {
            await message.member.voice.channel.join()
        }

        if (!queue.nowPlaying) {
            music.play(message, message.guild.voice.connection)
        }

    },
    permissions: [],
    requiredRoles: [],
}