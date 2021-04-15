const music = require("../../musicFunctions")
const serverConfig = require("../../serverConfigs/745662812335898806.json")

module.exports = {
    commands: ["nowplaying", "np", "song"],
    maxArgs: 0,
    callback: (message, arguments, text) => {
        if (serverConfig.musicQueue[0]) {
            music.getSongTitle(serverConfig.musicQueue[0]).then( (songTitle) => { message.reply(`Now playing: ${songTitle}`) })
        } else {
            message.reply("Nothing playing. Use `!play` to queue some music up.")
        }
        
    },
}