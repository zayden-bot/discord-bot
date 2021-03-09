const Discord = require("discord.js");
const config = require("../../serverConfigs/CKConfig.json")

const fs = require('fs');
const fileName = "CKConfig.json";

let ayarv = ""
for (let i = 0; i < config.ayasClub.length; i++) {
    ayarv = `${ayarv}\n${i+1}. ${config.ayasClub[i]}`
}

config.serverRules[93] = `**Aya's CK cheerleading sorority:**${ayarv}`

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
        let embed;

        if (!config.serverRules[id]) {
            message.reply(`There is no rule with the id ${id}`);
            return
        }
        
        if (id == 93) {
            embed = new Discord.MessageEmbed()
                .setTitle(`Rule ${id}`)
                .setDescription(`**${id}.** ${config.serverRules[id]}\n\n**Please read the rest of the rules in <#747430712617074718>!**`)
                .setColor("ff0000")
                .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
                .setImage("https://images-ext-1.discordapp.net/external/qSqe_sMdFj-dM1iuXadMclKHfV8CU5XGr2SFuIdKr_s/https/i.pinimg.com/originals/9d/af/64/9daf641ca935b02d992614ccee620e3f.gif")
        } else {
            embed = new Discord.MessageEmbed()
                .setTitle(`Rule ${id}`)
                .setDescription(`**${id}.** ${config.serverRules[id]}\n\n**Please read the rest of the rules in <#747430712617074718>!**`)
                .setColor("ff0000")
                .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
        }
        message.reply(embed);
    },
}