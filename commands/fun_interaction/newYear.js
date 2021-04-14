module.exports = {
    commands: ["newyear", "ny"],
    callback: (message, arguments, text) => {
        if (message.author.id == "211486447369322506") {
            message.channel.send(`Thank you Master Oscar for letting me survive to 2021 <:pandahappy:788512955641495592>`)
        }
        else {
            message.reply("Happy New Year")
        }
    },
}