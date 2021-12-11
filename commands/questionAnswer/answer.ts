import Discord from "discord.js"

module.exports = {
    commands: ["answer"],
    expectedArgs: "<id> <answer>",
    minArgs: 2,
    callback: (message: Discord.Message, args: string[], text: string) => {
        if (!message.guild) { return; }

        const id = Number(args.shift())

        if (typeof(id) !== "number") { 
            message.reply("Missing question ID")
            return
        }

        text = args.join(" ")

        const { questions } = require("./question")
        let question = questions[id]
        try {
            question.setAnswer(text, message.author.username)
        } catch {
            message.reply("Couldn't find question. Please try again")
            return
        }

        const embed = new Discord.MessageEmbed()
        .addField(`Question id: ${question.questionId}`, question.text)
        .addField(`Answered by ${question.answer.user}`, question.answer.text)

        const serverConfig = require(`../../server_configs/${message.guild.id}.json`)
        const channel = message.guild.channels.cache.get(serverConfig.channels.questionChannel)
        
        if (channel && channel.isText()) {
            channel.messages.fetch(question.messageId)
            .then(msg => { 
                msg.edit({embeds: [embed]})
                message.delete()
            })
        }

        try { question.user.send({embeds: [embed]}) }
        catch {}
        
    },
    permissions: ["MANAGE_MESSAGES"],
}
