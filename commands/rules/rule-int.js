const Discord = require("discord.js");
const config = require("../../serverConfigs/CKConfig.json")

const fs = require('fs');
const fileName = "CKConfig.json";

fs.writeFile("serverConfigs/CKConfig.json", JSON.stringify(config, null, 4), function writeJSON(err) {
    if (err) return console.log(err);
  });

module.exports = {
    commands: "rule",
    expectedArgs: "<number>",
    minArgs: 1,
    cooldown: 10,
    callback: (message, arguments, text) => {
        const id = arguments[0].toLowerCase();

        if (!config.serverRules[id]) {
            message.reply(`There is no rule with the id ${id}`);
            return
        }
        
        const embed = new Discord.MessageEmbed()
            .setTitle(`Rule ${id}`)
            .setDescription(`**${id}.** ${config.serverRules[id]}\n\n**Please read the rest of the rules in <#747430712617074718>!**`)
            .setColor("ff0000")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
        message.reply(embed);
    },
}