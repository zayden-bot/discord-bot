const Discord = require("discord.js");
const oscarsCult = [
    "<@709305430874259469>", // Leon
    "<@563985503834210304>", // Jeevant
    "<@124663881460219906>", // Slockie
    "<@304599215022276608>", // Fork
    "<@275401248331661313>", // Thyg
    "<@757213114843398145>", // Panty Lover
    "<@341545447199866880>", // Mark
    "<@516991142156435472>", // Abby
    "<@407157583409971202>", // Satan
]

const slockiesCult = [
    "<@211486447369322506>", // Oscar
    "<@516991142156435472>", // Abby
    "<@307239591755251722>", // JSlice
    "<@588577832905736192>" // LOYAL
]

const ayasCult = [
    "<@131797300006748160>", // AYA
    "<@342098139395653633>", // ASHARYA15
    "<@615128589918011393>", // JANY  
    "<@457249514542071819>" // Wall rider
]

const chloecock = [
    "<@516991142156435472>", // Abby
    "<@400649996556435458>", // Alo
    "<@444839364107698187>", // DaniD27
    "<@615128589918011393>", // Jany
    "<@709305430874259469>", // Leon
    "<@211486447369322506>", // Oscar
    "<@801133648170582078>", // Rudy
    "<@801044177138745394>", // SamZwill
    "<@124663881460219906>", // Slockie
    "<@747423760780623872>", // Steve
    "<@212376690574360588>", // Xander
    "<@275401248331661313>" // Thyg
]

let rv = ""
for (let i = 0; i < oscarsCult.length; i++) {
    rv = `${rv}\n${i+1}. ${oscarsCult[i]}`
}

let slockierv = ""
for (let i = 0; i < slockiesCult.length; i++) {
    slockierv = `${slockierv}\n${i+1}. ${slockiesCult[i]}`
}

let ayarv = ""
for (let i = 0; i < ayasCult.length; i++) {
    ayarv = `${ayarv}\n${i+1}. ${ayasCult[i]}`
}

let cockrv = ""
for (let i = 0; i < chloecock.length; i++) {
    cockrv = `${cockrv}\n${i+1}. ${chloecock[i]}`
}

let rules = {
    [0]: "Use common sense! If the staff are telling you to stop doing something, stop.",
    [1]: "This server is adult community (18+), by entering the server you agree that you are at least 18 years old. If you are suspected to be under the age of 18 you will be removed from the server.",
    [2]: "Be respectful. Opinions are fine, attacks are not. This includes but not limited to trolling, belittling, etc",
    [3]: "No discussing sensitive or controversial topics, eg religion and politics.",
    [4]: "No adveritising, links to promotional websites or affiliate links.",
    [5]: "This is not a dating service, don't treat it like one",
    [6]: "No spamming (including bot commands).",
    [7]: "We are an English only community. Please provide a translation with your message if it's not in English",
    [8]: "Pay attention to and respect our Staff, their decisions are final",
    [9]: "Don't link to anything against Discord ToS, such as sexualized jailbait/loli/shota.",
    [10]: "Don't ask other users for any kind of personal information.",
    [11]: "Make sure to read the pinned messages in each room.",
    [12]: "Stay on-topic in the respective channels",
    [13]: "Under no circumstances may you try to impersonate as one of the staff on this Discord server, whether it be on the development team, an admin or moderator.",
    [14]: "NSFW content is **ONLY** allowed in <#747428952577933424>. Posting Scat, Urine, Self Harm, Rape, Incest, Beastality, Drug use or Underaged content anywhere will get you immediatly banned. This is your only warning!",
    [27]: "Aya's the College Kings head cheerleader",
    [42]: `**Slockie's Vocal Cult:**${slockierv}`,
    [69]: "Whatever you do, DO NOT PING ME <:peperage:788512386205745162>",
    [93]: `**Aya's CK cheerleading sorority:**${ayarv}`,
    [420]: `**OscarSix's Cult:**${rv}`,
    [80085]: "Congratulations! You have found the secret rule. Winner: <@516991142156435472>",
    ["Hehe"]: "Matt thinks he's cool",
    ["Chloe"]: `**Chloe's Cult:**${cockrv}`
}

module.exports = {
    commands: "rule",
    expectedArgs: "<number>",
    minArgs: 1,
    cooldown: 10,
    callback: (message, arguments, text) => {
        const id = arguments[0]
        let embed;

        if (!rules[id]) {
            message.reply(`There is no rule with the id ${id}`);
            return
        }
        
        if (id == 93) {
            embed = new Discord.MessageEmbed()
                .setTitle(`Rule ${id}`)
                .setDescription(`**${id}.** ${rules[id]}\n\n**Please read the rest of the rules in <#747430712617074718>!**`)
                .setColor("ff0000")
                .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
                .setImage("https://images-ext-1.discordapp.net/external/qSqe_sMdFj-dM1iuXadMclKHfV8CU5XGr2SFuIdKr_s/https/i.pinimg.com/originals/9d/af/64/9daf641ca935b02d992614ccee620e3f.gif")
        } else {
            embed = new Discord.MessageEmbed()
                .setTitle(`Rule ${id}`)
                .setDescription(`**${id}.** ${rules[id]}\n\n**Please read the rest of the rules in <#747430712617074718>!**`)
                .setColor("ff0000")
                .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
        }
        message.reply(embed);
    },
}