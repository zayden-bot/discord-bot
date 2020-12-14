const Discord = require("discord.js");

module.exports = {
    commands: "rules",
    minArgs: 0,
    maxArgs: 0,
    callback: (message, arguments, text) => {
        const embed = new Discord.MessageEmbed()
            .setTitle(`𝒞𝑜𝓁𝓁𝑒𝑔𝑒 𝒦𝒾𝓃𝑔𝓈 𝒪𝒻𝒻𝒾𝒸𝒾𝒶𝓁 𝒮𝑒𝓇𝓋𝑒𝓇\n\n__**ꜱᴇʀᴠᴇʀ ʀᴜʟᴇꜱ**__`)
            .setDescription("**1.** Do not do anything which breaks the Discord ToS or Community Guidelines.\n\n**2.** Do not harass, bully or cause drama with other members within the discord.\n\n**3.** Racism, Sexism, Homophobia or any other offensive subject matters are strictly forbidden.\n**3a.** Talking about Politics, Religion and other sensitive subjects are also forbidden\n\n**4.** Do not spam. This includes Images, Repeatedly Asking Questions or spamming emojis.\n\n**5.** No Advertising. This includes but not limited to Discord Servers or other websites that aren't related to College Kings. Offical websites to other games are allowed in <#772516507041005618>\n\n**6.** Do not threaten to DDoS or dox someone, it is also prohibited to discuss these topics or share information regarding either topic (As well as discuss information gained via a dox).\n\n**7.** Do not post any NSFW pictures outside of an NSFW marked channel. Gore, Loli, Shota and other Extreme NSFW content is prohibited.\n\n**8.** This is an English-only server.\n\n**9.** Stay on-topic in the respective channels\n\n**10.** Respect our staff team, their decisions are final.\n\n\n**If you do not agree/abide with these rules, you will get kicked or banned from the server. Here at College Kings you are to follow our Discord's Community Guidelines.**")
            .setColor("ff0000")
            .setImage("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg?width=1440&height=360")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
        message.reply(embed)
    },
    permissions: ["ADMINISTRATOR"],
}