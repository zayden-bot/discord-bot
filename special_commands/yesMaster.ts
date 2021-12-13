import Discord from "discord.js"

module.exports = function (message: Discord.Message) {
    if (!message.client.user) { return; }

    if (message.content.toLowerCase() == `shut up <@!${message.client.user.id}>` && message.author.id == "211486447369322506") {
        message.channel.send(`Sorry Master ${message.author.username}, will try better next time <:pepeKMS:788514824203141181>`)
    }
}
