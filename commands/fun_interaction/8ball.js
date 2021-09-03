module.exports = {
    commands: ["8ball"],
    expectedArgs: "<question>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const member = message.member
        const randomNumber = Math.floor(Math.random() * 3)

        if (randomNumber == 0 || (member.id == "211486447369322506" && message.content.endsWith('?'))) {
            message.reply("Yes")
        }
        else if (randomNumber == 1 || member.id == "211486447369322506") {
            message.reply("No")
        }
        else {
            message.reply("Maybe")
        }
    },
}