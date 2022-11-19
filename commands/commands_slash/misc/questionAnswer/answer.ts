import Discord from "discord.js"
import {ChannelType} from 'discord-api-types/v10';
import {getServer, IServer} from "../../../../models/server";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("answer")
        .setDescription("Answer a user's question")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.ManageMessages)
        .addNumberOption(option =>
            option.setName("id")
                .setDescription("The member's question ID"))
        .addStringOption(option =>
            option.setName("answer")
                .setDescription("Your answer")),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild || !(interaction.member instanceof Discord.GuildMember)) {
            return;
        }

        const server: IServer = await getServer(interaction.guild.id)

        const id = interaction.options.getInteger("id")
        if (!id) {
            return interaction.reply({content: "Invalid ID", ephemeral: true})
        }

        const text = interaction.options.getString("answer", true)

        let question = server.questions[id]
        if (!question) {
            return interaction.reply({content: "Couldn't find question. Please try again", ephemeral: true})
        }

        question.answer = {
            text: text,
            username: interaction.member.displayName
        }

        const embed = new Discord.EmbedBuilder()
            .addFields([
                {name: `Question id: ${question.questionId}`, value: question.text},
                {name: `Answered by ${interaction.member.displayName}`, value: question.answer.text}
            ])

        const channel = await interaction.guild.channels.fetch(server.channels.questionChannel)

        if (channel && channel.type == ChannelType.GuildText && question.messageId) {
            const questionMessage = await channel.messages.fetch(question.messageId)
            questionMessage.edit({embeds: [embed]})
        }

        const questionUser = await interaction.client.users.fetch(question.userId)
        await Promise.all([
            questionUser.send({embeds: [embed]}),
            interaction.reply({content: "Question answered successfully", ephemeral: true}),
            server.save()
        ])
    },
}
