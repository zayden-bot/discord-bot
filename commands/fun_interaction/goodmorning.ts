import Discord from "discord.js";

module.exports = {
    commands: ["goodmorning", "gm"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message: Discord.Message, args: string[], text: string) => {
        let member: Discord.GuildMember | undefined;
        if (message.mentions.members) { member = message.mentions.members.first(); }
        if (!member) { member = message.member as Discord.GuildMember }

        const imgConfig = require("../../configs/image_config.json")
        let arrayId = "Global";
        if (message.author.id in imgConfig.goodMorningImgs) { arrayId = message.author.id }

        const imgId = Math.floor(Math.random() * imgConfig.goodMorningImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`Good Morning, ${member.displayName}`)
            .setImage(imgConfig.goodMorningImgs[arrayId][imgId])

        message.channel.send({embeds: [embed]})
    },
}
