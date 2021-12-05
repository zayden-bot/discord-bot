module.exports = {
    commands: ["newyear", "ny"],
    callback: (message, arguments, text) => {
        const serverConfig = require(`../../serverConfigs/${message.guild.id}.json`)

        if (serverConfig.masters.includes(message.author.id)) {
            message.channel.send(`Thank you Master ${message.author.username} for letting me survive to 2021 <:pandahappy:788512955641495592>`)
        } else { message.reply("Happy New Year!") }
    },
}
