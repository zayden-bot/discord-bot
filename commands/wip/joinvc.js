const Blacklist = require("../../blacklist.js");

module.exports = {
    commands: ["joinVC"],
    expectedArgs: "<Voice Channel ID>",
    minArgs: 0,
    callback: (message, arguments, text) => {
        if (Blacklist.isProtectedUser(message.author.id)) {
            const voiceChannel = message.member.voice.channel;
            if (!voiceChannel) {
                return message.reply("You have to be in a voice channel!");
            } else {
                voiceChannel.join().then((connection) => {
                    message.reply("Joined the channel!");
                }).catch(e => {
                    console.error(e);
                });
            }
        }
    },
}