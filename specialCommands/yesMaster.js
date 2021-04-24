module.exports = function(message) {
    const serverConfig = require(`../serverConfigs/${message.guild.id}.json`)

    if (message.content.toLowerCase() == `shut up <@!${message.client.user.id}>` && serverConfig.masters.includes(message.author.id)) {
        message.channel.send(`Sorry Master ${message.author.username}, will try better next time <:pepeKMS:788514824203141181>`)
    }
}
