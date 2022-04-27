import Discord from "discord.js";
import {Server} from "../../models/servers/server";

module.exports = {
    commands: ["give_star", "gs"],
    expectedArgs: "<user> [text]",
    minArgs: 1,
    callback: async (message: Discord.Message) => {
        const author = message.member;
        const member = message.mentions.members?.first();

        if (!message.guild || !author) {
            return;
        }

        if (!member) {
            return message.reply("No member mentioned.");
        }
        if (member.id == author.id) {
            return message.reply("You idiot...");
        }

        const common = require("../../common")
        common.user_config_setup(message);

        const member_config = require(`../../user_configs/${member.id}.json`);
        const author_config = require(`../../user_configs/${author.id}.json`);
        const server = await Server.findOne({id: message.guild.id}).exec()

        if (author_config["number_of_stars"] <= 0 && !author.roles.cache.has(server.roles.staffRole)) {
            await message.reply("Unable. You have no gold stars to give.");
            return;
        }

        if (!author.roles.cache.has(server.roles.staffRole)) {
            author_config["number_of_stars"] -= 1;
        }
        author_config["given_stars"] += 1;
        member_config["number_of_stars"] += 1;
        member_config["received_stars"] += 1;

        common.update_configs(message, member_config, author_config);

        const embed = new Discord.MessageEmbed()
            .setTitle(`⭐ NEW GOLDEN STAR ⭐`)
            .setDescription(`<@${member.id}> received a golden star from <@${author.id}> for a total of ${member_config["number_of_stars"]} stars`);

        message.channel.send({embeds: [embed]});
    },
}
