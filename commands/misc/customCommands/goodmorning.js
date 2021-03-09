const Discord = require("discord.js")

const GMImages = {
    "Global": [
        "https://cdn.discordapp.com/attachments/796455862968909904/805281646253047818/Riley_sleeping.png",
        "https://cdn.discordapp.com/attachments/796455862968909904/805307740121464832/addtext_com_MDAyNTIwNDI0NzI.png",
        "https://cdn.discordapp.com/attachments/796455862968909904/805313219798630419/Julia_streching_1.png",
        "https://cdn.discordapp.com/attachments/796455862968909904/806365147123548190/Emily_Good_Morning_1_1.png",
        "https://cdn.discordapp.com/attachments/796455862968909904/806388589793378305/Aubrey_Riley_good_morning_1.png"
    ],
    "615128589918011393": [
        "https://i.imgur.com/2iEytRE.png"
    ]
}

module.exports = {
    commands: ["goodmorning", "gm"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let member = message.author.username
        if (text) { member = message.mentions.members.first().user.username }

        let arrayId = "Global"
        if (message.author.id in GMImages) { arrayId = message.author.id }

        const imgId = Math.floor(Math.random() * GMImages[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`Good Morning, ${member}`)
            .setImage(GMImages[arrayId][imgId])

        message.channel.send(embed)
    },
}