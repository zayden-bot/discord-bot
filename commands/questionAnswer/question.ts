import Discord from "discord.js"

class Question {
    text: string;
    user: Discord.User;
    questionId: number;
    messageId: string | undefined;
    answer: Answer | undefined;
    
    constructor(text: string, user: Discord.User) {
        this.text = text
        this.user = user
        this.questionId = questions.length
        this.messageId = undefined;
        this.answer = undefined;
    }

    setMessageId(messageId: string) {
        this.messageId = messageId
    }

    setAnswer(text: string, user: Discord.User) {
        this.answer = new Answer(text, user)
    }
}

class Answer {
    text: string;
    user: Discord.User;

    constructor(text: string, user: Discord.User) {
        this.text = text
        this.user = user
    }
}

let questions: Question[] = []

module.exports = {
    commands: ["question", "ask"],
    expectedArgs: "<question>",
    minArgs: 1,
    callback: (message: Discord.Message, args: string[], text: string) => {
        if (!message.guild) { return }

        const serverConfig = require(`../../server_configs/${message.guild.id}.json`)

        const question = new Question(text, message.author)
        questions.push(question)

        const embed = new Discord.MessageEmbed()
        .addField(`Question id: ${question.questionId}`, question.text)

        const channel = message.guild.channels.cache.get(serverConfig.channels.questionChannel)
        if (channel && channel.isText()) {
            channel.send({embeds: [embed]}).then((message: Discord.Message) => { question.setMessageId(message.id) })
        }
    },
    questions: questions
}
