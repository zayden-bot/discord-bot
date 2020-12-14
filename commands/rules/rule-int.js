const Discord = require("discord.js");

module.exports = {
    commands: "rule",
    minArgs: 1,
    maxArgs: 1,
    callback: (message, arguments, text) => {
        switch (arguments[0]) {
            case 1:
                var ruleTitle = "Rule 1"
                var ruleDescription = "**1.** Do not do anything which breaks the Discord ToS or Community Guidelines."
                break;
            case 2:
                var ruleTitle = "Rule 1"
                var ruleDescription = "**1.** Do not do anything which breaks the Discord ToS or Community Guidelines."
                break;
        }

        const embed = new Discord.MessageEmbed()
            .setTitle(ruleTitle)
            .setDescription(ruleDescription)
            .setColor("ff0000")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
        message.reply(embed)
    },
    permissions: "ADMINISTRATOR",
}
