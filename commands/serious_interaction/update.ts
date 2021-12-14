import Discord from "discord.js"
import { servers } from "../../server"

function msToTime(ms: number) {
    const days = Math.floor(ms / (1000 * 60 * 60 * 24));
    const hours = Math.floor((ms % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
    const minutes = Math.floor((ms % (1000 * 60 * 60)) / (1000 * 60));
    const seconds = Math.floor((ms % (1000 * 60)) / (1000));
    return [days, hours, minutes, seconds]
}

module.exports = {
    commands: ["update"],
    callback: (message: Discord.Message, args: string[], text: string) => {
        if (!message.guild || !message.member) { return; }

        const server = servers[message.guild.id]
        if (message.channel.id != "776139754408247326" && !message.member.roles.cache.has(server.roles.moderationRole)) {
            message.reply("You're an idiot. Use <#776139754408247326> for commands.").then(msg => {
                setTimeout(() => msg.delete(), 5000);
            });
            message.delete();
            return;
        }

        const currentTimeMS = new Date().getTime()

        // Patreon update
        let patreonOutput = "";
        let patreonDateMS = 0;
        let patreonUpdateMS = 0;
        const patreonDate = new Date(server.gameVersions.patreonUpdate)

        if (server.gameVersions.patreonUpdate == "") {
            patreonOutput = "No public date set."
        } else {
            patreonDateMS = patreonDate.getTime()
            patreonUpdateMS = patreonDateMS - currentTimeMS
        }

        if (patreonUpdateMS < 0) {
            patreonOutput = "RELEASED"
        } else if (patreonUpdateMS > 0) {
            const day = patreonDate.toLocaleString("en-GB", { day:"numeric" })
            const month = patreonDate.toLocaleString("en-GB", { month:"long" })
    
            const [ days, hours, minutes, seconds ] = msToTime(patreonUpdateMS)

            patreonOutput = `${day} ${month}\n${days}d ${hours}h ${minutes}m ${seconds}s`
        }

        // Steam update
        let steamOutput = "";
        let steamDateMS = 0;
        let steamUpdateMS = 0;
        const steamDate = new Date(server.gameVersions.steamUpdate)

        if (server.gameVersions.steamUpdate == "") {
            steamOutput = "No public date set."
        } else {
            steamDateMS = steamDate.getTime()
            steamUpdateMS = steamDateMS - currentTimeMS
        }

        if (steamUpdateMS < 0) {
            steamOutput = "RELEASED"
        } else if (steamUpdateMS > 0) {
            const day = steamDate.toLocaleString("en-GB", { day:"numeric" })
            const month = steamDate.toLocaleString("en-GB", { month:"long" })
    
            const [ days, hours, minutes, seconds ] = msToTime(steamUpdateMS)

            steamOutput = `${day} ${month}\n${days}d ${hours}h ${minutes}m ${seconds}s`
        }

        // Send update message
        const embed = new Discord.MessageEmbed()
            .setTitle(`Next Update`)
            .setColor("#ff0000")
            .setDescription("If you are interested in the next update, read below:")
            .addField(`__Patreon Release__ (${server.gameVersions.patreon_version})`, patreonOutput, true)
            .addField(`__Steam Release__ (${server.gameVersions.steam_version})`, steamOutput, true)
            .setImage("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg?width=1440&height=360")
            .setURL("https://www.patreon.com/collegekings")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
            .setFooter("https://www.patreon.com/collegekings")

        message.reply({embeds: [embed]})
    },
}
