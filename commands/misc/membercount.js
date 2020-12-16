module.exports = {
    commands: ["membercount"],
    minArgs: 0,
    maxArgs: 0,
    callback: (message, arguments, text) => {
        message.channel.send(`**${message.guild.memberCount}** total members`)
    },
}