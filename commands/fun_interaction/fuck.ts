import Discord from "discord.js"

module.exports = {
    commands: ["fuck"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message: Discord.Message, args: string[], text: string) => {
        if (!(message.channel.type == "GUILD_TEXT")) return;

        if (!message.channel.nsfw) {
            message.reply("This command can only be used in nsfw channels")
            return
        }
        
        // Check and get mentioned member
        const mentions = message.mentions.members
        let member = message.author.username
        if ((mentions) && (mentions.size > 0)) {
            member = mentions.first()?.user.username || ""
        }

        const imgConfig = require("../../configs/imgConfig.json")
        const imgId = Math.floor(Math.random() * imgConfig.fuckingImgs.length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`${message.author.username} fucks ${member}`)
            .setImage(imgConfig.fuckingImgs[imgId])
            .setColor("#FFC0CB")

        message.channel.send({
            embeds: [embed],
        })
    }
}
