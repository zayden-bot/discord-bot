import Discord from "discord.js"
import { servers } from "../../../server";

module.exports = {
    commands: ["support"],
    expectedArgs: "<id>",
    minArgs: 1,
    callback: (message: Discord.Message, args: string[], text: string) => {
        const guild = message.guild
        if (!guild) { return; }

        const server = servers[guild.id]

        const id = text;
        const answer = server.supportAnswers[id.toLowerCase()];
        if (!answer) { return message.reply(`There is no support answer for ID: ${id}`); }

        const embed = new Discord.MessageEmbed()
        .setTitle(`Support ID: ${id[0].toUpperCase() + id.slice(1)}`)
        .setDescription(answer)
        .setColor("#ff0000")
        .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png");
        
        message.channel.send({embeds: [embed]});
    },
    requiredRoles: ["Support Team"]
}
