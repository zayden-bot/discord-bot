import Discord from "discord.js"

module.exports = {
    commands: ["8ball"],
    expectedArgs: "<question>",
    minArgs: 1,
    callback: async (message: Discord.Message) => {
        const member = message.member;
        if (!member) {
            return;
        }

        const randomNumber = Math.floor(Math.random() * 3);
        const responses = require("../../configs/8ball_responses.json");

        let randomIndex: number;
        switch (randomNumber) {
            case 0:
                randomIndex = Math.floor(Math.random() * responses.Yes.length)
                await message.reply(responses.Yes[randomIndex])
                break;
            case 1:
                randomIndex = Math.floor(Math.random() * responses.No.length)
                await message.reply(responses.No[randomIndex])
                break;
            case 2:
                randomIndex = Math.floor(Math.random() * responses.Maybe.length)
                await message.reply(responses.Maybe[randomIndex])
                break;
        }
    }
}