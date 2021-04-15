const Discord = require("discord.js");
const config = require("../../serverConfigs/745662812335898806.json")

const PatreonUpdate = new Date(config.patreonUpdate)
// const SteamUpdate = new Date(config.steamUpdate)

const PatreonUpdateMS = PatreonUpdate.getTime()
let SteamUpdateMS;
if (typeof(SteamUpdate) !== "undefined") {
    SteamUpdateMS = SteamUpdate.getTime()
}

function msToTime(ms) {
    const days = Math.floor(ms / (1000 * 60 * 60 * 24));
    const hours = Math.floor((ms % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
    const minutes = Math.floor((ms % (1000 * 60 * 60)) / (1000 * 60));
    const seconds = Math.floor((ms % (1000 * 60)) / (1000));
    return [days, hours, minutes, seconds]
}

module.exports = {
    commands: ["update"],
    callback: (message, arguments, text) => {
        const currentTime = new Date().getTime()
        let newSteamUpdate;
        if (typeof(SteamUpdate) !== "undefined") {
            newSteamUpdate = SteamUpdateMS - currentTime
        }
        const newPatreonUpdate = PatreonUpdateMS - currentTime

        let patreonOutput, steamOutput;
        
        if (typeof(PatreonUpdate) == "undefined") {
            patreonOutput = "No public date set."
        } else if (newPatreonUpdate < 0) {
            patreonOutput = "RELEASED"
        } else {
            const day = PatreonUpdate.toLocaleString("en-GB", { day:"numeric" })
            const month = PatreonUpdate.toLocaleString("en-GB", { month:"long" })
    
            const [ days, hours, minutes, seconds ] = msToTime(newPatreonUpdate)

            patreonOutput = `${day}th ${month}\n${days}d ${hours}h ${minutes}m ${seconds}s`
        }

        if (typeof(SteamUpdate) == "undefined") {
            steamOutput = "Late May or Early June"
        } else if (newSteamUpdate < 0) {
            steamOutput = "RELEASED"
        } else {
            const day = SteamUpdate.toLocaleString("en-GB", { day:"numeric" })
            const month = SteamUpdate.toLocaleString("en-GB", { month:"long" })

            const [ days, hours, minutes, seconds ] = msToTime(newSteamUpdate)
    
            steamOutput = `${day}th ${month}\n${days}d ${hours}h ${minutes}m ${seconds}s`
        }

        const embed = new Discord.MessageEmbed()
            .setTitle("Next College Kings Update (0.9)")
            .setColor("ff0000")
            .setDescription("If you are interested in the next update, read below:")
            .addField("__Patreon Release__ ($10)", patreonOutput, true)
            .addField("__Steam Release__ ($5 - $15)", steamOutput, true)
            .setImage("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg?width=1440&height=360")
            .setURL("https://www.youtube.com/watch?v=rIelf_KPybE")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
            .setFooter("https://www.patreon.com/collegekings")

        message.reply(embed)
    },
}
