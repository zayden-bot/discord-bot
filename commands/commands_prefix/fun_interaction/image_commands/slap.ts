import Discord from "discord.js";
import {getImage} from "./image_cmd_base";

module.exports = {
    commands: ["slap"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: async (message: Discord.Message) => {
        const member = message.mentions.members?.first() || message.member
        if (!member) {
            return;
        }

        const image = await getImage(message.author, "slap")

        if (message.member?.id == "393046490966130688" && member.id == "211486447369322506") { // Sondwich
            const embed = new Discord.MessageEmbed()
                .setTitle("Oscar slaps Sondwich")
                .setImage(image);
            try {
                await message.reply({content: "Bitch! You don't slap me.", embeds: [embed]})
            } catch {
                message.channel.send({content: "Bitch! You don't slap me.", embeds: [embed]})
            }
            return;
        }

        const embed = new Discord.MessageEmbed()
            .setTitle(`${message.author.username} slaps ${member.displayName}`)
            .setImage(image);

        message.channel.send({embeds: [embed]})
    },
}
