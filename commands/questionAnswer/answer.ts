// noinspection JSIgnoredPromiseFromCall

import Discord from "discord.js"

module.exports = {
    commands: ["answer"],
    expectedArgs: "<id> <answer>",
    minArgs: 2,
    callback: async (message: Discord.Message, args: string[], text: string) => {
        if (!message.guild) {
            return;
        }

        const id = Number(args.shift())

        if (isNaN(id)) {
            message.reply("Missing question ID")
            return
        }

        text = args.join(" ")

        const {questions} = require("./question")
        let question = questions[id]
        try {
            question.setAnswer(text, message.author.username)
        } catch {
            await message.reply("Couldn't find question. Please try again")
            return
        }

        const embed = new Discord.MessageEmbed()
            .addField(`Question id: ${question.questionId}`, question.text)
            .addField(`Answered by ${question.answer.user}`, question.answer.text)

        const serverConfig = require(`../../server_configs/${message.guild.id}.json`)
        const channel = message.guild.channels.cache.get(serverConfig.channels.questionChannel)

        if (channel && channel.isText()) {
            try {
                const questionMessage = await channel.messages.fetch(question.messageId)
                questionMessage.edit({embeds: [embed]})
                await message.delete()
            } catch {
                message.reply("Failed to edit question. Did the original question get deleted?")
            }
        }

        try {
            question.user.send({embeds: [embed]})
        } catch {
        }

    },
    permissions: ["MANAGE_MESSAGES"],
}
