const Discord = require("discord.js");
const serverConfig = require("../serverConfigs/CKConfig.json")

let serverRules = ""
for (rule in serverConfig.serverRules) {
    if (rule == "69" || rule == "80085") { continue }
    serverRules += `**${rule}.** ${serverConfig.serverRules[rule]}\n\n`
}
// console.log(serverRules)

module.exports = async (client, channelId) => {
    const channel = await client.channels.fetch(channelId)

    const updatedRules = new Discord.MessageEmbed()
        .setTitle(`𝒞𝑜𝓁𝓁𝑒𝑔𝑒 𝒦𝒾𝓃𝑔𝓈 𝒪𝒻𝒻𝒾𝒸𝒾𝒶𝓁 𝒮𝑒𝓇𝓋𝑒𝓇\n\n__**ꜱᴇʀᴠᴇʀ ʀᴜʟᴇꜱ**__`)
        .setDescription(`${serverRules}`)
        .setColor("ff0000")
        .setImage("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg?width=1440&height=360")
        .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")

    channel.messages.fetch("788539168980336701").then((message) => { message.edit(updatedRules) })
    // Rules Message ID ^
}