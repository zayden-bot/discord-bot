import Discord from "discord.js"

module.exports = {
    commands: ["8ball"],
    expectedArgs: "<question>",
    minArgs: 1,
    callback: (message: Discord.Message, args: string[], text: string) => {
        const member = message.member;
        if (!member) { return; }

        const randomNumber = Math.floor(Math.random() * 3);
        const responses = require("../../configs/8ball_responses.json");

        let randomIndex: number;
        switch (randomNumber) {
            case 0:
                randomIndex = Math.floor(Math.random() * responses.Yes.length)
                message.reply(responses.Yes[randomIndex])
                break;
            case 1:
                randomIndex = Math.floor(Math.random() * responses.No.length)
                message.reply(responses.No[randomIndex])
                break;
            case 2:
                randomIndex = Math.floor(Math.random() * responses.Maybe.length)
                message.reply(responses.Maybe[randomIndex])
                break;
            default:
                message.reply("<@211486447369322506> Error: Unknown random number")
        }
    }
}