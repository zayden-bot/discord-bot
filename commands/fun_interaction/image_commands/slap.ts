import Discord from "discord.js";

module.exports = {
    commands: ["slap"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: async (message: Discord.Message) => {
        const member = message.mentions.members?.first() || message.member
        if (!member) {
            return;
        }

        if (member.id == "211486447369322506") {
            await message.reply(`Sorry ${message.author} you cannot slap Master Six`)
            return;
        }

        const imageConfig = require("../../../configs/image_config.json")
        let arrayId = "global";
        if (message.author.id in imageConfig.slapImages) {
            arrayId = message.author.id
        }

        const imgId = Math.floor(Math.random() * imageConfig.slapImages[arrayId].length)

        let embed;
        if (message.member?.id == "393046490966130688" && member.id == "211486447369322506") { // Sondwich
            embed = new Discord.MessageEmbed()
                .setTitle("Oscar slaps Sondwich")
                .setImage(imageConfig.slapImages[arrayId][imgId]);
            try {
                await message.reply({content: "Bitch! You don't slap me.", embeds: [embed]})
            } catch {
                message.channel.send({content: "Bitch! You don't slap me.", embeds: [embed]})
            }
            return;
        }

        embed = new Discord.MessageEmbed()
            .setTitle(`${message.author.username} slaps ${member.displayName}`)
            .setImage(imageConfig.slapImages[arrayId][imgId]);

        message.channel.send({embeds: [embed]})
    },
}
