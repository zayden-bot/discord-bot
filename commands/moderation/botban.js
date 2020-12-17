const Discord = require("discord.js")
let commandBase = require("../command-base")

module.exports = {
    commands: ["botban"],
    minArgs: 1,
    maxArgs: 1,
    callback: (message, arguments, text) => {
        const { mentions } = message

        // message.reply("Command is in development")

        const embed = new Discord.MessageEmbed()
            .setDescription(`***${mentions.users.first().username}* was bot banned**`)
            .setColor("#ff0000")

        message.reply(embed)
    },
    requiredRoles: ["Security"],
}

