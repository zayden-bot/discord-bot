module.exports = {
    commands: ["join", "j"],
    maxArgs: 0,
    disabled: true,
    callback: (message, arguments, text) => {
        if (!message.member.voice.channel) {
            message.reply("You have to be connected to a voice channel before you can use this command!")
            return
        }

        if (!message.guild.voice || !message.guild.voice.channelID) {
            message.member.voice.channel.join().then(connection => message.reply(`Joined ${connection.channel.name}`))
        }

    },
}