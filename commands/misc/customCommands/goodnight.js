const Discord = require("discord.js")

const GNImages = [
    "https://cdn.discordapp.com/attachments/796455862968909904/805281646253047818/Riley_sleeping.png",
]

module.exports = {
    commands: ["goodnight", "gn"],
    callback: (message, arguments, text) => {
        const imgId = Math.floor(Math.random() * (GNImages.length -1))
        const embed = new Discord.MessageEmbed()
            .setTitle(`Good Night, ${message.author.username}`)
            .setImage(GNImages[imgId])
        // console.log(kissingImages[imgId])
        message.channel.send(embed)
    },
}