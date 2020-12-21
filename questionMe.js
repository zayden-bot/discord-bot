module.exports = (client) => {
    client.on("message", (message) => {
        if (message.mentions.users.size) {
            if (message.mentions.users.first().id == "787490197943091211" && message.content.slice(-1) == "?") {
                if (Math.floor(Math.random() * 2) == 0) {
                        message.reply("Yes");
                } else {
                    message.reply("No");
                }
            }
        }
    })
}