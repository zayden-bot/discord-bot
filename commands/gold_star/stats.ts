import Discord from "discord.js"

module.exports = {
    commands: ["stats"],
    expectedArgs: "<user>",
    maxArgs: 1,
    callback: (message: Discord.Message, args: string[], text: string) => {
        let member: Discord.GuildMember | undefined;
        if (message.mentions.members) { member = message.mentions.members.first(); }
        if (!member) { member = message.member as Discord.GuildMember; }

        const commom = require("../../common")
        commom.user_config_setup(message);
        const member_config = require(`../../user_configs/${member.id}.json`);

        const embed = new Discord.MessageEmbed()
            .setTitle(`${member.user.username} Stats`)
            .addField("Number of Stars", member_config.number_of_stars.toString(), true)
            .addField("Given Stars", member_config.given_stars.toString(), true)
            .addField("Received Stars", member_config.received_stars.toString(), true)

        message.channel.send({embeds: [embed]})
    },
}