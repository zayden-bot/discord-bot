const Discord = require("discord.js")

var kissingImages = [
    "https://cdn.weeb.sh/images/B1NwJg9Jz.gif",
    "https://cdn.weeb.sh/images/SyY0j6Ov-.gif",
    "https://cdn.weeb.sh/images/rJ_U2p_Pb.gif",
    "https://cdn.weeb.sh/images/SydfnauPb.gif",
    "https://cdn.weeb.sh/images/r1VWnTuPW.gif",
    "https://cdn.weeb.sh/images/ryceu6V0W.gif",
    "https://cdn.weeb.sh/images/HJmunTOw-.gif",
    "https://cdn.weeb.sh/images/rkFSiEedf.gif",
    "https://cdn.weeb.sh/images/HkZyXs3A-.gif",
    "https://media.giphy.com/media/l2Je2M4Nfrit0L7sQ/giphy.gif",
]

module.exports = {
    commands: ["kiss"],
    expectedArgs: "<user>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const imgId = Math.floor(Math.random() * kissingImages.length)
        const member = message.mentions.members.first()
        const embed = new Discord.MessageEmbed()
            .setTitle(`${message.author.username} kisses ${member.user.username}`)
            .setImage(kissingImages[imgId])
            .setColor("FFC0CB")
        // console.log(kissingImages[imgId])
        message.channel.send(embed)
    },
}