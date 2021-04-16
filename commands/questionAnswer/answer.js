const Discord = require("discord.js")

module.exports = {
    commands: ["answer"],
    expectedArgs: "<id> <answer>",
    minArgs: 2,
    callback: async (message, arguments, text) => {
        const serverConfig = require(`../../serverConfigs/${message.guild.id}.json`)
        const { questions } = require("./question")

        const id = Number(arguments.shift())

        if (typeof(id) == "undefined") {
            message.reply("Missing question ID")
            return
        }

        text = arguments.join(" ")

        let question = questions[id]
        try {
            question.setAnswer(text, message.author.username)
        } catch {
            message.reply("Couldn't find question. Please try again")
            return
        }

        const embed = new Discord.MessageEmbed()
        .addField(`Question ID: ${question.questionId}`, question.text)
        .addField(`Answered by ${question.answer.user}`, question.answer.text)

        const channel = await message.guild.channels.cache.get(serverConfig.questionChannel)
        channel.messages.fetch(question.messageId).then(msg => { 
            msg.edit(embed)
            message.delete()
        })
        
        // embed.setTitle("You're question has been answered.")

        try { question.user.send(embed) }
        catch {}
        
    },
    requireRoles: ["Security"]
}
