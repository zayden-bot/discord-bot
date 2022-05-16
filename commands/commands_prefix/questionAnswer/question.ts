import Discord from "discord.js"
import {IQuestion, IServer} from "../../../models/server";

module.exports = {
    commands: ["question", "ask"],
    expectedArgs: "<question>",
    minArgs: 1,
    callback: async (message: Discord.Message, server: IServer, args: string[], text: string) => {
        if (!message.guild) {
            return
        }

        const questionChannel = await message.guild.channels.fetch(server.channels.questionChannel)
        if (!questionChannel || !questionChannel.isText()) {
            return;
        }

        const question: IQuestion = {
            text: text,
            userId: message.author.id,
            questionId: server.questions.length,
            messageId: null,
            answer: null
        }

        const embed = new Discord.MessageEmbed()
            .addField(`Question id: ${question.questionId}`, text)

        questionChannel.send({embeds: [embed]})
            .then((msg) => {
                question.messageId = msg.id
                server.questions.push(question)
                server.save()
            })
            .catch((error) => {
                message.reply(error)
            })
    }
}
