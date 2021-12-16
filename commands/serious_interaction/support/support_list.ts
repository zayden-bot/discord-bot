import Discord from "discord.js"
import { servers } from "../../../server";

module.exports = {
    commands: ["support_list"],
    minArgs: 0,
    maxArgs: 0,
    callback: (message: Discord.Message, args: string[], text: string) => {
        const guild = message.guild
        if (!guild) { return; }

        const server = servers[guild.id]

        const embed = new Discord.MessageEmbed()
        .setTitle("List of support options")
        .setColor("#ff0000")
        .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png");
        
        for (const [id, answer] of Object.entries(server.supportAnswers)) {
            embed.addField(`ID: ${id}`, answer);
        }

        message.channel.send({embeds: [embed]});
    },
    requiredRoles: ["Support Team"]
}
