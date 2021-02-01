const Discord = require("discord.js")

const GNImages = [
    "https://cdn.discordapp.com/attachments/796455862968909904/805291737878364180/Lauren_sleeping.png",
    "https://cdn.discordapp.com/attachments/796455862968909904/805300252986703883/addtext_com_MjM1NTI5NDE1NTA.png",
]

module.exports = {
    commands: ["goodnight", "gn"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let member = message.author.username
        if (text) { member = message.mentions.members.first().user.username }
        const imgId = Math.floor(Math.random() * GNImages.length)
        const embed = new Discord.MessageEmbed()
            .setTitle(`Good Night, ${member}`)
            .setImage(GNImages[imgId])
        // console.log(kissingImages[imgId])
        message.channel.send(embed)
    },
}