module.exports = {
    commands: ["join", "j"],
    permissionError: "Command is currently in development. Limited to staff use only.",
    maxArgs: 0,
    callback: (message, arguments, text) => {
        if (!message.member.voice.channel) {
            message.reply("You have to be connected to a voice channel before you can use this command!")
            return
        }

        if (!message.guild.voiceConnection) {
            message.member.voice.channel.join().then(connection => message.reply(`Joined ${connection.channel.name}`))
        }

    },
    permissions: [],
    requiredRoles: ["Security"],
}