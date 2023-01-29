import Discord from "discord.js"
import {ChannelType} from 'discord-api-types/v10';
import {IQuestion} from "../../../../models/server_settings/QuestionSchema";
import {getConnection} from "../../../../servers";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("answer")
        .setDescription("Answer a user's question")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.ManageMessages)
        .addIntegerOption(option =>
            option.setName("id")
                .setDescription("The member's question ID")
                .setRequired(true))
        .addStringOption(option =>
            option.setName("answer")
                .setDescription("Your answer")
                .setRequired(true)),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild || !(interaction.member instanceof Discord.GuildMember)) {
            return;
        }
        const conn = getConnection(interaction.guild.id)

        const id = interaction.options.getInteger("id", true)
        const text = interaction.options.getString("answer", true)

        const questionChannel = await conn.model<IQuestion>("Channels").findOne({category: "question"})
        if (!questionChannel) {
            return interaction.reply({
                content: "Couldn't find question channel. Please contact server admin",
                ephemeral: true
            })
        }

        const question = await conn.model<IQuestion>("Questions").findOne({questionId: id})
        if (!question) {
            return interaction.reply({content: "Couldn't find question. Please try again", ephemeral: true})
        }

        question.answer = {
            text: text,
            userId: interaction.member.id
        }

        const embed = new Discord.EmbedBuilder()
            .addFields([
                {name: `Question id: ${question.questionId}`, value: question.text},
                {name: `Answered by ${interaction.member.displayName}`, value: question.answer.text}
            ])

        const channel = await interaction.guild.channels.fetch(questionChannel.id)

        if (channel && channel.type == ChannelType.GuildText && question.messageId) {
            const questionMessage = await channel.messages.fetch(question.messageId)
            questionMessage.edit({embeds: [embed]})
        }

        const questionUser = await interaction.client.users.fetch(question.userId)
        await Promise.all([
            questionUser.send({embeds: [embed]}),
            interaction.reply({content: "Question answered successfully", ephemeral: true}),
            question.save()
        ])
    },
}
