const Discord = require("discord.js");

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
        const config = require(`../../serverConfigs/${message.guild.id}`)
        const currentTimeMS = new Date().getTime()

        // Patreon update
        let patreonOutput = "";
        let patreonDateMS = 0;
        let patreonUpdateMS = 0;
        const patreonDate = new Date(config.patreonUpdate)

        if (config.patreonUpdate == "") {
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
        const steamDate = new Date(config.steamUpdate)

        if (config.steamUpdate == "") {
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
            .setTitle(`Next Update (${config.game_version})`)
            .setColor("ff0000")
            .setDescription("If you are interested in the next update, read below:")
            .addField("__Patreon Release__ ($10)", patreonOutput, true)
            .addField("__Steam Release__", steamOutput, true)
            .setImage("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg?width=1440&height=360")
            .setURL("https://www.patreon.com/collegekings")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")
            .setFooter("https://www.patreon.com/collegekings")

        message.reply(embed)
    },
}
