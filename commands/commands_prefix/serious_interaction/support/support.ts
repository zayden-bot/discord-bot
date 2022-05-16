import Discord from "discord.js";
import {IServer} from "../../../../models/server";

module.exports = {
    commands: ["support"],
    expectedArgs: "<id>",
    minArgs: 1,
    callback: async (message: Discord.Message, server: IServer, args: string[], text: string) => {
        const guild = message.guild
        if (!guild) {
            return;
        }

        const id = text.toLowerCase();
        const answer = server.supportAnswers.get(id);
        if (!answer) {
            await message.reply(`There is no support answer for ID: ${id}`);
            return;
        }

        const embed = new Discord.MessageEmbed()
            .setTitle(`Support ID: ${id[0].toUpperCase() + id.slice(1)}`)
            .setDescription(answer)
            .setColor("#ff0000")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png");

        await Promise.all([
            message.channel.send({embeds: [embed]}),
            message.delete()
        ])
    },
    requiredRoles: ["Support Team", "Staff"]
}
