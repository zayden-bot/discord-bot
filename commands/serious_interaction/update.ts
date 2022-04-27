import Discord from "discord.js";
import {servers} from "../../servers";

module.exports = {
    commands: ["update"],
    callback: async (message: Discord.Message) => {
        if (!message.guild || !message.member) {
            return;
        }

        const server = servers[message.guild.id]

        if (message.channel.id != "776139754408247326" && !message.member.roles.cache.has(server.roles.staffRole)) {
            const msg = await message.reply("You're an idiot. Use <#776139754408247326> for commands.")
            await message.delete()
            setTimeout(() => msg.delete(), 5000);
            return;
        }

        const patreonUpdateTime = server.gameVersions.patreonUpdate || "Unknown"
        const steamUpdateTime = server.gameVersions.steamUpdate || "Unknown"

        // Send update message
        const embed = new Discord.MessageEmbed()
            .setTitle(`Next Update`)
            .setColor("#ff0000")
            .setDescription("If you are interested in the next update, read below:")
            .addField(`__Patreon Release__ (${server.gameVersions.patreonVersion})`, patreonUpdateTime, true)
            .addField(`__Steam Release__ (${server.gameVersions.steamVersion})`, steamUpdateTime, true)
            .setImage("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg?width=1440&height=360")
            .setURL("https://www.patreon.com/collegekings")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
            .setFooter({text: "https://www.patreon.com/collegekings"})

        await message.reply({embeds: [embed]});
    },
}
