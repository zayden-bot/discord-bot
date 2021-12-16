import Discord from "discord.js";
import { servers } from "../../server";

module.exports = {
    commands: "rule",
    expectedArgs: "<number>",
    minArgs: 1,
    cooldown: 10,
    callback: (message: Discord.Message, args: string[], text: string) => {
        const guild = message.guild
        if (!guild) { return; }

        const server = servers[message.guild.id]

        const id = args[0].toLowerCase();

        if (!server.serverRules[id] && !server.hidden.rules[id]) {
            return message.reply(`There is no rule with the id ${id}`);
        }

        const rule: string = server.serverRules[id] || server.hidden.rules[id]

        
        const embed = new Discord.MessageEmbed()
        .setTitle(`Rule ${id}`)
        .setDescription(`**${id}.** ${rule}\n\n**Please read the rest of the rules in <#747430712617074718>!**`)
        .setColor("#ff0000")
        .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png");
        
        message.channel.send({embeds: [embed]});

    },
}
