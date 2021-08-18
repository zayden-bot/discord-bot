const Discord = require("discord.js")
const fs = require("fs")

module.exports = {
    commands: ["give_star", "gs"],
    expectedArgs: "<user> [text]",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const default_config = {
            "number_of_stars": 0,
            "given_stars": 0,
            "received_stars": 0
        }

        const member = message.mentions.members.first();
        if (!member) {
            message.reply("No member mentioned.")
            return;
        }
        
        const author = message.member;
        const server_config = require(`../../serverConfigs/${message.guild.id}`)
        let member_config;
        let author_config;

        if (!fs.existsSync(`./user_configs/${member.id}.json`)) {
            console.log("Writing member config")
            fs.writeFileSync(`./user_configs/${member.id}.json`, JSON.stringify(default_config, null, 4), function writeJSON(err) {
                if (err) { return console.log(err); }
            });
        }
        member_config = require(`../../user_configs/${member.id}`)

        if (!fs.existsSync(`./user_configs/${author.id}.json`)) {
            console.log("Writing author config")
            fs.writeFileSync(`./user_configs/${author.id}.json`, JSON.stringify(default_config, null, 4), function writeJSON(err) {
                if (err) { return console.log(err); }
            });
        }
        author_config = require(`../../user_configs/${author.id}`)

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

        fs.writeFileSync(`./user_configs/${author.id}.json`, JSON.stringify(author_config, null, 4), function writeJSON(err) {
            if (err) { return console.log(err); }
        });
        fs.writeFileSync(`./user_configs/${member.id}.json`, JSON.stringify(member_config, null, 4), function writeJSON(err) {
            if (err) { return console.log(err); }
        });

        const embed = new Discord.MessageEmbed()
        .setTitle(`⭐ NEW GOLDEN STAR ⭐`)
        .setDescription(`<@${member.id}> recieved a golden star from <@${author.id}> for a total of ${member_config.number_of_stars}`)

        message.channel.send(embed)

    },
}
