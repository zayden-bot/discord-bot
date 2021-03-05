const Discord = require("discord.js")

const GNImages = {
    "Global": [
        "https://cdn.discordapp.com/attachments/796455862968909904/805291737878364180/Lauren_sleeping.png",
        "https://cdn.discordapp.com/attachments/796455862968909904/805300252986703883/addtext_com_MjM1NTI5NDE1NTA.png",
        "https://cdn.discordapp.com/attachments/796455862968909904/805991914222518322/addtext_com_MjE0MzQzNTAxOQ.png",
    ],
    "615128589918011393": [
        "https://i.imgur.com/ZvATozx.png"
    ]
}

module.exports = {
    commands: ["goodnight", "gn"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let member = message.author.username
        if (text) { member = message.mentions.members.first().user.username }

        let arrayId = "Global"
        if (message.author.id in GNImages) { arrayId = message.author.id }

        const imgId = Math.floor(Math.random() * GNImages[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`Good Morning, ${member}`)
            .setImage(GNImages[arrayId][imgId])

        message.channel.send(embed)
    },
}