const Discord = require("discord.js");
const rules = {
    [1]: "This server is adult community (18+), by entering the server you agree that you are at least 18 years old. If you are suspected to be under the age of 18 you will be removed from the server.",
    [2]: "Be respectful. Opinions are fine, attacks are not. This includes but not limited to trolling, belittling, etc",
    [3]: "Avoid discussing controversial topics, eg religion and politics.",
    [4]: "This is not a dating service, don't treat it like one",
    [5]: "No spamming (including bot commands).",
    [6]: "We are an English only community. Please provide a translation with your message if it's not in English",
    [7]: "Pay attention to and respect our Staff, their decisions are final",
    [8]: "Don't link to anything against Discord ToS, such as sexualized jailbait/loli/shota.",
    [9]: "Don't ask other users for any kind of personal information.",
    [10]: "Make sure to read the pinned messages in each room.",
    [11]: "Stay on-topic in the respective channels",
    [12]: "Respect our staff team, their decisions are final.",
    [13]: "Under no circumstances may you try to impersonate as one of the staff on this Discord server, whether it be on the development team, an admin or moderator.",
    [14]: "NSFW content is **ONLY** allowed in <#747428952577933424>. Posting Scat, Urine, Self Harm, Rape, Incest, Beastality, Drug use or Underaged content anywhere will get you immediatly banned. This is your only warning!"
}

module.exports = {
    commands: "rule",
    expectedArgs: "<number>",
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
}
