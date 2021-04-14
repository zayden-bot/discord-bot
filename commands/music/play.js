const fs = require("fs")

const music = require("../../musicFunctions")

const serverConfig = require("../../serverConfigs/CKConfig.json")

module.exports = {
    commands: ["play", "p"],
    expectedArgs: "<youtube link>",
    permissionError: "Command is currently in development. Limited to staff use only.",
    minArgs: 1,
    callback: (message, arguments, text) => {
        if (!message.member.voice.channel) {
            message.reply("You have to be connected to a voice channel before you can use this command!")
            return
        }

        if (!arguments[0].startsWith("https://www.youtube.com/")) {
            message.reply("Music links must be from YouTube. (https://www.youtube.com/)")
            return
        }

        // Adds to queue
        serverConfig.musicQueue.push(arguments[0])
        music.getSongTitle(arguments[0]).then(songTitle => message.channel.send(`Added ${songTitle} to the Queue.`))

        fs.writeFile("./serverConfigs/CKConfig.json", JSON.stringify(serverConfig, null, 4), function writeJSON(err) {
            if (err) { return console.log(err); }
        });

        if (!message.guild.voice) {
            message.member.voice.channel.join().then(connection => { music.play(message, connection) })
        }
    },
    permissions: [],
    requiredRoles: ["Security"],
}