import Discord from "discord.js";

module.exports = {
    commands: ["kiss"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message: Discord.Message, args: string[], text: string) => {
        let member: Discord.GuildMember | undefined;
        if (message.mentions.members) {
            member = message.mentions.members.first();
        }
        if (!member) {
            member = message.member as Discord.GuildMember
        }

        const imageConfig = require("../../configs/image_config.json")
        let arrayId = "global";
        if (message.author.id in imageConfig.kissingImgs) {
            arrayId = message.author.id
        }

        const imgId = Math.floor(Math.random() * imageConfig.kissingImgs[arrayId].length)

        const embed = new Discord.MessageEmbed()
            .setTitle(`${message.author.username} kisses ${member.displayName}`)
            .setImage(imageConfig.kissingImgs[arrayId][imgId])
            .setColor("#FFC0CB")

        message.channel.send({embeds: [embed]})
    },
}
