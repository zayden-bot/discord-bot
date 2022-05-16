import Discord from "discord.js";
import {IServer} from "../../../models/server";

module.exports = {
    commands: "rule",
    expectedArgs: "<number>",
    minArgs: 1,
    callback: (message: Discord.Message, server: IServer, args: string[]) => {
        const guild = message.guild
        if (!guild) {
            return;
        }

        const id = args[0].toLowerCase();
        const rule = server.serverRules[Number(id)] || server.hidden.rules.get(id)

        if (!rule) {
            return message.reply(`There is no rule with the id ${id}`);
        }

        const embed = new Discord.MessageEmbed()
            .setTitle(`Rule ${id}`)
            .setDescription(`**${id}.** ${rule}\n\n**Please read the rest of the rules in <#747430712617074718>!**`)
            .setColor("#ff0000")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png");

        message.channel.send({embeds: [embed]});

    },
}
