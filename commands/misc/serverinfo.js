module.exports = {
    commands: ["serverinfo", "membercount"],
    minArgs: 0,
    maxArgs: 0,
    callback: (message, arguments, text) => {
        client.guilds.cache.forEach((guild) => {
            message.channel.send(`**${guild.memberCount}** total members`)
        })
    },
}