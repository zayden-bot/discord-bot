module.exports = function(message) {
    try { var serverConfig = require(`../server_configs/${message.guild.id}.json`) }
    catch { var serverConfig = require("../server_configs/privateMessage.json") }

    if (message.content.toLowerCase() == `shut up <@!${message.client.user.id}>` && message.author.id == "211486447369322506") {
        message.channel.send(`Sorry Master ${message.author.username}, will try better next time <:pepeKMS:788514824203141181>`)
    }
}
