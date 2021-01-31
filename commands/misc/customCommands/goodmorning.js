const Discord = require("discord.js")

const GMImages = [
    "https://cdn.discordapp.com/attachments/796455862968909904/805281646253047818/Riley_sleeping.png",
]

module.exports = {
    commands: ["goodmorning", "gm"],
    callback: (message, arguments, text) => {
        const imgId = Math.floor(Math.random() * (GMImages.length -1))
        const embed = new Discord.MessageEmbed()
            .setTitle(`Good Morning, ${message.author.username}`)
            .setImage(GMImages[imgId])
        // console.log(kissingImages[imgId])
        message.channel.send(embed)
    },
}