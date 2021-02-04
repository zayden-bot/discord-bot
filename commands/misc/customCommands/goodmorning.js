const Discord = require("discord.js")

const GMImages = [
    "https://cdn.discordapp.com/attachments/796455862968909904/805281646253047818/Riley_sleeping.png",
    "https://cdn.discordapp.com/attachments/796455862968909904/805307740121464832/addtext_com_MDAyNTIwNDI0NzI.png",
    "https://cdn.discordapp.com/attachments/796455862968909904/805313219798630419/Julia_streching_1.png",
    "https://cdn.discordapp.com/attachments/796455862968909904/806365147123548190/Emily_Good_Morning_1_1.png",
    "https://cdn.discordapp.com/attachments/796455862968909904/806388589793378305/Aubrey_Riley_good_morning_1.png"
]

module.exports = {
    commands: ["goodmorning", "gm"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let member = message.author.username
        if (text) { member = message.mentions.members.first().user.username }
        const imgId = Math.floor(Math.random() * GMImages.length)
        const embed = new Discord.MessageEmbed()
            .setTitle(`Good Morning, ${member}`)
            .setImage(GMImages[imgId])
        // console.log(kissingImages[imgId])
        message.channel.send(embed)
    },
}