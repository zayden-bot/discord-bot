module.exports = function(message) {
    try { var serverConfig = require(`../serverConfigs/${message.guild.id}.json`) }
    catch { var serverConfig = require("../serverConfigs/privateMessage.json") }

    if (message.content.toLowerCase() == `shut up <@!${message.client.user.id}>` && serverConfig.masters.includes(message.author.id)) {
        message.channel.send(`Sorry Master ${message.author.username}, will try better next time <:pepeKMS:788514824203141181>`)
    }
}
