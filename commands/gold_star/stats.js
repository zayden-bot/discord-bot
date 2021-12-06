const Discord = require("discord.js")
const commom = require("../../common")

module.exports = {
    commands: ["stats"],
    expectedArgs: "[user]",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let member = message.mentions.members.first();
        if (!member) { member = message.member; }

        commom.user_config_setup(message);
        const member_config = require(`../../user_configs/${member.id}.json`);

        const embed = new Discord.MessageEmbed()
            .setTitle(`${member.user.username} Stats`)
            .addFields(
                { name: "Number of Stars", value: member_config["number_of_stars"], inline: true },
                { name: "Given Stars", value: member_config["given_stars"], inline: true },
                { name: "Received Stars", value: member_config["received_stars"], inline: true },
            )

        message.channel.send({embeds: [embed]})
    },
}