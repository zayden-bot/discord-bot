import Discord from "discord.js"

module.exports = {
    command: "questionMe",
    callback: (message: Discord.Message) => {
        if (message.mentions.users.first()?.id == message.client.user?.id && message.content.slice(-1) == "?") {
            if (Math.floor(Math.random() * 2)) {
                message.reply("Yes").then();
            } else {
                message.reply("No").then();
            }
        }
    }
}