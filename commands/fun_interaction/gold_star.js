const Discord = require("discord.js")
const common = require("../../common")

module.exports = {
    commands: ["give_star", "gs"],
    expectedArgs: "<user> [text]",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const member = message.mentions.members.first();
        if (!member) {
            message.reply("No member mentioned.")
            return;
        }
        const author = message.member;
        
        common.user_config_setup(message);

        member_config = require(`../../user_configs/${member.id}.json`)
        author_config = require(`../../user_configs/${author.id}.json`)
        server_config = require(`../../serverConfigs/${message.guild.id}.json`)

        if (author_config["number_of_stars"] <= 0 && !author.roles.cache.has(server_config.staffRoles)) {
            message.reply("Unable. You have no gold stars to give.");
            return;
        }

        if (!author.roles.cache.has(server_config.staffRoles)) {
            author_config["number_of_stars"] -= 1;
        }
        author_config["given_stars"] += 1;
        member_config["number_of_stars"] += 1;
        member_config["received_stars"] += 1;

        common.update_user_configs(message)

        const embed = new Discord.MessageEmbed()
        .setTitle(`⭐ NEW GOLDEN STAR ⭐`)
        .setDescription(`<@${member.id}> recieved a golden star from <@${author.id}> for a total of ${member_config.number_of_stars}`)

        message.channel.send(embed)

    },
}
