const Discord = require("discord.js");

module.exports = async (client, channelId) => {
    const channel = await client.channels.fetch(channelId)

    const updatedRules = new Discord.MessageEmbed()
        .setTitle(`𝒞𝑜𝓁𝓁𝑒𝑔𝑒 𝒦𝒾𝓃𝑔𝓈 𝒪𝒻𝒻𝒾𝒸𝒾𝒶𝓁 𝒮𝑒𝓇𝓋𝑒𝓇\n\n__**ꜱᴇʀᴠᴇʀ ʀᴜʟᴇꜱ**__`)
        .setDescription(`
            **0.** Use common sense! If the staff are telling you to stop doing something, stop.\n
            **1.** This server is adult community (18+), by entering the server you agree that you are at least 18 years old. If you are suspected to be under the age of 18 you will be removed from the server.\n
            **2.** Be respectful. Opinions are fine, attacks are not. This includes but not limited to trolling, belittling, etc\n
            **3.** No discussing sensitive or controversial topics, eg religion and politics.\n
            **4.** No advertising, links to promotional websites or affiliate links.\n
            **5.** This is not a dating service, don't treat it like one\n
            **6.** No spamming (including bot commands).\n
            **7.** We are an English only community. Please provide a translation with your message if it's not in English\n
            **8.** Pay attention to and respect our Staff, their decisions are final\n
            **9.** Don't link to anything against Discord ToS, such as sexualized jailbait/loli/shota.\n
            **10.** Don't ask other users for any kind of personal information.\n
            **11.** Make sure to read the pinned messages in each room.\n
            **12.** Stay on-topic in the respective channels\n
            **13.** Under no circumstances may you try to impersonate as one of the staff on this Discord server, whether it be on the development team, an admin or moderator.\n
            **14.** NSFW content is **ONLY** allowed in <#747428952577933424>. Posting Scat, Urine, Self Harm, Rape, Incest, Beastality, Drug use or Underaged content anywhere will get you immediatly banned. This is your only warning!\n\n
            **If you do not agree/abide with these rules, you will get kicked or banned from the server. Here at College Kings you are to follow our Discord's Community Guidelines.**
        `)
        .setColor("ff0000")
        .setImage("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg?width=1440&height=360")
        .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")

    channel.messages.fetch("788539168980336701").then((message) => { message.edit(updatedRules) })
    // Rules Message ID ^
}