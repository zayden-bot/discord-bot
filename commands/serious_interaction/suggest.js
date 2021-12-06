const Discord = require("discord.js")

module.exports = {
    commands: ["suggest", "suggestion"],
    expectedArgs: "<text>",
    permissionError: "",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const config = require("../../server_configs/745662812335898806.json")

        const embed = new Discord.MessageEmbed()
            .setTitle(`From: ${message.author.username}`)
            .setDescription(text)

        let channel = message.guild.channels.cache.get(config.suggestionChannel)
        channel.send({embeds: [embed]}).then(function(message) {
            message.react("👍")
            message.react("👎")
        })
    },
}
