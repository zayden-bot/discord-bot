const Discord = require("discord.js")

module.exports = {
    commands: ["bug", "bugs"],
    callback: (message, arguments, text) => {
        const embed = new Discord.MessageEmbed()
        .setTitle("College Kings v0.6 Reported Bugs")
        .addFields(
            { name: "Broken/Corrupt Download File",  value:"Status: Problem Found" }
        )
        .setThumbnail(message.guild.iconURL())

        message.channel.send(embed)
    },
}