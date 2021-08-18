const Discord = require("discord.js")
const fs = require("fs")

module.exports = {
    commands: ["stats"],
    expectedArgs: "[user]",
    maxArgs: 1,
    callback: (message, arguments, text) => {
        let member = message.mentions.members.first();
        if (!member) { member = message.member; }

        if (!fs.existsSync(`./user_configs/${member.id}.json`)) {
            console.log("Writing member config")
            fs.writeFileSync(`./user_configs/${member.id}.json`, JSON.stringify(default_config, null, 4), function writeJSON(err) {
                if (err) { return console.log(err); }
            });
        }
        member_config = require(`../../user_configs/${member.id}`)

        const embed = new Discord.MessageEmbed()
            .setTitle(`${member.user.username} Stats`)
            .addFields(
                { name: "Number of Stars", value: member_config["number_of_stars"], inline: true },
                { name: "Given Stars", value: member_config["given_stars"], inline: true },
                { name: "Recieved Stars", value: member_config["received_stars"], inline: true },
            )

        message.channel.send(embed)
    },
}