module.exports = (client) => {
    client.on("message", (message) => {
        if (message.mentions.users.has("747423760780623872") && !message.author.bot) {
            message.reply("Please don't @mention Steve. He will respond to messages when he has a change, if you have a suggestion please use `!suggest`.")
        }
    })
}