const Discord = require("discord.js")
const imgConfig = require("../../configs/imgConfig.json")

module.exports = {
    commands: ["fuck"],
    expectedArgs: "<user>",
    maxArgs: 1,
    cooldown: 43200,
    callback: (message, arguments, text) => {
        if (!message.channel.nsfw) {
            message.reply("This command can only be used in nsfw channels")
            return
        }

        let member;
        try { member = message.mentions.members.first().user.username }
        catch { member = message.author.username }

        let arrayId = "Global"
        if (message.author.id in imgConfig.fuckingImgs) { arrayId = message.author.id }
        else {
            try {
                if (message.mentions.members.first().user.id in imgConfig.fuckingImgs) { arrayId = message.mentions.members.first().user.id }
            } catch {}
        }

        const imgId = Math.floor(Math.random() * imgConfig.fuckingImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`${message.author.username} fucks ${member}`)
            .setImage(imgConfig.fuckingImgs[arrayId][imgId])
            .setColor("FFC0CB")

        message.channel.send(embed)
    }
}
