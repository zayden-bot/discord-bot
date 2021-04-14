module.exports = (client) => {
    client.on("message", (message) => {
        if (message.content.toLowerCase() == "shut up <@!787490197943091211>" && message.author.id == "211486447369322506") {
            message.channel.send("Sorry Master Oscar, will try better next time <:pepeKMS:788514824203141181>")
        }
    })
}
