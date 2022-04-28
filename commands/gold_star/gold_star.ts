import Discord from "discord.js";
import {IServer} from "../../models/server";
import {getUserConfig, IUserConfig} from "../../models/user-config";

module.exports = {
    commands: ["give_star", "gs"],
    expectedArgs: "<user> [text]",
    minArgs: 1,
    callback: async (message: Discord.Message, server: IServer) => {
        const author = message.member;
        const member = message.mentions.members?.first();

        if (!message.guild || !author || !member) {
            return;
        }
        if (member.id == author.id) {
            return message.reply("You idiot...");
        }


        const member_config: IUserConfig = await getUserConfig(member.id)
        const author_config: IUserConfig = await getUserConfig(author.id)

        if (author_config.stars.numberOfStars <= 0 && !author.roles.cache.has(server.roles.moderationRole)) {
            await message.reply("Unable. You have no gold stars to give.");
            return;
        }

        if (!author.roles.cache.has(server.roles.moderationRole)) {
            author_config.stars.numberOfStars -= 1;
        }
        author_config.stars.givenStars += 1;
        member_config.stars.numberOfStars += 1;
        member_config.stars.receivedStars += 1;

        const embed = new Discord.MessageEmbed()
            .setTitle(`⭐ NEW GOLDEN STAR ⭐`)
            .setDescription(`<@${member.id}> received a golden star from <@${author.id}> for a total of ${member_config.stars.numberOfStars} stars`);

        message.channel.send({embeds: [embed]});
        await Promise.all([
            member_config.save(),
            author_config.save()
        ])
    },
}
