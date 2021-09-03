const responses = require("../../configs/8ballResponses.json")

module.exports = {
    commands: ["8ball"],
    expectedArgs: "<question>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const member = message.member
        const randomNumber = Math.floor(Math.random() * 3)

        if (randomNumber == 0 || (member.id == "211486447369322506" && message.content.endsWith('?'))) {
            const randomIndex = Math.floor(Math.random() * responses.Yes.length)
            message.reply(responses.Yes[randomIndex])
        }
        else if (randomNumber == 1 || member.id == "211486447369322506") {
            const randomIndex = Math.floor(Math.random() * responses.No.length)
            message.reply(responses.No[randomIndex])
        }
        else {
            const randomIndex = Math.floor(Math.random() * responses.Maybe.length)
            message.reply(responses.Maybe[randomIndex])
        }
    },
}