import Discord from "discord.js";
import {IServer} from "../../models/server";

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

        const member_config = require(`../../user_configs/${member.id}.json`);
        const author_config = require(`../../user_configs/${author.id}.json`);

        if (author_config["number_of_stars"] <= 0 && !author.roles.cache.has(server.roles.moderationRole)) {
            await message.reply("Unable. You have no gold stars to give.");
            return;
        }

        if (!author.roles.cache.has(server.roles.moderationRole)) {
            author_config["number_of_stars"] -= 1;
        }
        author_config["given_stars"] += 1;
        member_config["number_of_stars"] += 1;
        member_config["received_stars"] += 1;

        const embed = new Discord.MessageEmbed()
            .setTitle(`⭐ NEW GOLDEN STAR ⭐`)
            .setDescription(`<@${member.id}> received a golden star from <@${author.id}> for a total of ${member_config["number_of_stars"]} stars`);

        message.channel.send({embeds: [embed]});
    },
}
