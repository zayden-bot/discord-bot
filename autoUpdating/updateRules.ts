import Discord from "discord.js";
import {IRule} from "../models/server_settings/RulesSchema";

export default async function updateRules(rules: IRule[], channel: Discord.Channel, messageId?: string) {
    let serverRules = ""
    for (const rule of rules) {
        serverRules += `**${rule.ruleId}.** ${rule.ruleText}\n\n`
    }

    const embed = new Discord.EmbedBuilder()
        .setTitle(`𝒞𝑜𝓁𝓁𝑒𝑔𝑒 𝒦𝒾𝓃𝑔𝓈 𝒪𝒻𝒻𝒾𝒸𝒾𝒶𝓁 𝒮𝑒𝓇𝓋𝑒𝓇\n\n__**ꜱᴇʀᴠᴇʀ ʀᴜʟᴇꜱ**__`)
        .setDescription(`${serverRules}`)
        .setColor("#ff0000")
        .setImage("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg")
        .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png")

    if (!channel.isTextBased() || !messageId)
        return

    const msg = await channel.messages.fetch(messageId)
    msg.edit({embeds: [embed]})
}
