const Discord = require("discord.js")

class Question {
    constructor(text, user) {
        this.text = text
        this.user = user
        this.questionId = questions.length
    }

    setMessageId(messageId) {
        this.messageId = messageId
    }

    setAnswer(text, user) {
        this.answer = new Answer(text, user)
    }
}

class Answer {
    constructor(text, user) {
        this.text = text
        this.user = user
    }
}

let questions = []

module.exports = {
    commands: ["question", "questions"],
    expectedArgs: "<question>",
    minArgs: 1,
    callback: (message, arguments, text) => {
        const serverConfig = require(`../../serverConfigs/${message.guild.id}.json`)

        question = new Question(text, message.author)
        questions.push(question)

        const embed = new Discord.MessageEmbed()
        .addField(`Question id: ${question.questionId}`, question.text)

        let channel = message.guild.channels.cache.get(serverConfig.questionChannel)
        channel.send(embed).then(message => { question.setMessageId(message.id) })
    },
    questions: questions
}
