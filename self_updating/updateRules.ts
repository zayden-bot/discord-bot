import Discord from "discord.js";

module.exports = async function (client: Discord.Client, channelId: string) {
    const serverConfig = require("../server_configs/745662812335898806.json")
    
    const channel = await client.channels.fetch(channelId)
    if (!channel || !channel.isText()) { return console.error("Invalid channel id")}

    let serverRules = ""
    for (const rule in serverConfig.serverRules) {
        serverRules += `**${rule}.** ${serverConfig.serverRules[rule]}\n\n`
    }

    const embed = new Discord.MessageEmbed()
        .setTitle(`𝒞𝑜𝓁𝓁𝑒𝑔𝑒 𝒦𝒾𝓃𝑔𝓈 𝒪𝒻𝒻𝒾𝒸𝒾𝒶𝓁 𝒮𝑒𝓇𝓋𝑒𝓇\n\n__**ꜱᴇʀᴠᴇʀ ʀᴜʟᴇꜱ**__`)
        .setDescription(`${serverRules}`)
        .setColor("#ff0000")
        .setImage("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg?width=1440&height=360")
        .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")

    channel.messages.fetch("788539168980336701").then((message) => { message.edit({embeds: [embed]}) })
    // Rules Message ID ^
}
