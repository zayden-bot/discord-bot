const Discord = require("discord.js");
const rules = {
    [1]: "Do not do anything which breaks the Discord ToS or Community Guidelines.",
    [2]: "Do not harass, bully or cause drama with other members within the discord.",
    [3]: "Racism, Sexism, Homophobia or any other offensive subject matters are strictly forbidden.",
    ['3a']: "Talking about Politics, Religion and other sensitive subjects are also forbidden.",
    [4]: "Do not spam. This includes Images, Repeatedly Asking Questions or spamming emojis.",
    [5]: "No Advertising. This includes but not limited to Discord Servers or other websites that aren't related to College Kings. Offical websites to other games are allowed in <#787822407733608448>.",
    [6]: "Do not threaten to DDoS or dox someone, it is also prohibited to discuss these topics or share information regarding either topic (As well as discuss information gained via a dox).",
    [7]: "Do not post any NSFW pictures outside of an NSFW marked channel. Gore, Loli, Shota and other Extreme NSFW content is prohibited.",
    [8]: "This is an English-only server.",
    [9]: "Stay on-topic in the respective channels.",
    [10]: "Respect our staff team, their decisions are final."
}

module.exports = {
    commands: "rule",
    minArgs: 1,
    maxArgs: 1,
    callback: (message, arguments, text) => {
        const id = arguments[0]

        const embed = new Discord.MessageEmbed()
            .setTitle(`Rule ${id}`)
            .setDescription(`**${id}.** ${rules[id]}\n\n**Please read the rest of the rules in <#747430712617074718>!**`)
            .setColor("ff0000")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
        message.reply(embed)
    },
    permissions: "ADMINISTRATOR",
}