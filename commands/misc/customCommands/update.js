const Discord = require("discord.js");
const PatreonUpdate = new Date("12 Feb, 2021 19:00:00")
const SteamUpdate = new Date("5 Mar, 2021 19:00:00")

const PatreonUpdateMS = PatreonUpdate.getTime()
const SteamUpdateMS = SteamUpdate.getTime()

module.exports = {
    commands: ["update"],
    callback: (message, arguments, text) => {
        const currentTime = new Date().getTime()
        const newSteamUpdate = SteamUpdateMS - currentTime
        const newPatreonUpdate = PatreonUpdateMS - currentTime

        let patreonOutput, steamOutput;
        
        if (newPatreonUpdate < 0) {
            patreonOutput = "RELEASED"
        } else {
            const day = PatreonUpdate.toLocaleString("en-GB", { day:"numeric" })
            const month = PatreonUpdate.toLocaleString("en-GB", { month:"long" })

            const days = Math.floor(newPatreonUpdate / (1000 * 60 * 60 * 24));
            const hours = Math.floor((newPatreonUpdate % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
            const minutes = Math.floor((newPatreonUpdate % (1000 * 60 * 60)) / (1000 * 60));
            const seconds = Math.floor((newPatreonUpdate % (1000 * 60)) / (1000));
    
            patreonOutput = `${day}th ${month}\n${days}d ${hours}h ${minutes}m ${seconds}s`
        }

        if (newSteamUpdate < 0) {
            steamOutput = "RELEASED"
        } else {
            const day = SteamUpdate.toLocaleString("en-GB", { day:"numeric" })
            const month = SteamUpdate.toLocaleString("en-GB", { month:"long" })

            const days = Math.floor(newSteamUpdate / (1000 * 60 * 60 * 24));
            const hours = Math.floor((newSteamUpdate % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
            const minutes = Math.floor((newSteamUpdate % (1000 * 60 * 60)) / (1000 * 60));
            const seconds = Math.floor((newSteamUpdate % (1000 * 60)) / (1000));
    
            steamOutput = `${day}th ${month}\n${days}d ${hours}h ${minutes}m ${seconds}s`
        }

        const embed = new Discord.MessageEmbed()
            .setTitle("Next College Kings Update (0.7)")
            .setColor("ff0000")
            .setDescription("If you are interested in the next update, read below:")
            .addField("__Patreon Release ($10)__", patreonOutput, true)
            .addField("__Public/Steam Release__", steamOutput, true)
            .setImage("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg?width=1440&height=360")
            .setURL("https://youtu.be/ANc4IieXxRk")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
            .setFooter("https://www.patreon.com/collegekings")

        message.reply(embed)
    },
}
