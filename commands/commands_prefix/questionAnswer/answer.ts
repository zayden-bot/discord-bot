import Discord from "discord.js"
import {IServer} from "../../../models/server";
import {ChannelType} from 'discord-api-types/v10';

module.exports = {
    commands: ["answer"],
    expectedArgs: "<id> <answer>",
    minArgs: 2,
    callback: async (message: Discord.Message, server: IServer, args: string[], text: string) => {
        if (!message.guild) {
            return;
        }

        const id = Number(args.shift())
        if (isNaN(id)) {
            await message.reply("Missing question ID")
            return
        }

        text = args.join(" ")

        let question = server.questions[id]
        if (!question) {
            await message.reply("Couldn't find question. Please try again")
            return
        }

        question.answer = {
            text: text,
            username: message.member?.displayName || message.author.username
        }

        const embed = new Discord.EmbedBuilder()
            .addFields([
                {name: `Question id: ${question.questionId}`, value: question.text},
                {name: `Answered by ${message.author.username}`, value: question.answer.text}
            ])

        const channel = await message.guild.channels.fetch(server.channels.questionChannel)

        if (channel && channel.type == ChannelType.GuildText && question.messageId) {
            const questionMessage = await channel.messages.fetch(question.messageId)
            questionMessage.edit({embeds: [embed]})
            message.delete().catch()
        }

        const questionUser = await message.client.users.fetch(question.userId)
        questionUser.send({embeds: [embed]}).catch()
        await server.save()
    },
    permissions: ["MANAGE_MESSAGES"],
}
