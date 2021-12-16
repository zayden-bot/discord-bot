import Discord from "discord.js";

module.exports = {
    commands: ["kiss"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message: Discord.Message, args: string[], text: string) => {
        let member: Discord.GuildMember | undefined;
        if (message.mentions.members) { member = message.mentions.members.first(); }
        if (!member) { member = message.member as Discord.GuildMember }

        const imgConfig = require("../../configs/image_config.json")
        let arrayId = "Global";
        if (message.author.id in imgConfig.kissingImgs) { arrayId = message.author.id }

        const imgId = Math.floor(Math.random() * imgConfig.kissingImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`${message.author.username} kisses ${member.displayName}`)
            .setImage(imgConfig.kissingImgs[arrayId][imgId])
            .setColor("#FFC0CB")

        message.channel.send({embeds: [embed]})
    },
}
