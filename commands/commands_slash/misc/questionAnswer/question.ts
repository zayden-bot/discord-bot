import Discord from "discord.js"
import {ChannelType} from "discord-api-types/v10"
import {getConnection} from "../../../../servers";
import {IQuestion} from "../../../../models/server_settings/QuestionSchema";
import {IChannel} from "../../../../models/server_settings/ChannelSchema";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("question")
        .setDescription("Ask a question to be answered by a Team Member")
        .addStringOption(option =>
            option.setName("question")
                .setDescription("Your question")
                .setRequired(true)),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild || !(interaction.member instanceof Discord.GuildMember)) {
            return;
        }

        const conn = getConnection(interaction.guild.id)
        const questionsCollection = conn.model<IQuestion>("Questions")
        const questionChannel = await interaction.guild.channels.fetch((await conn.model<IChannel>("Channels").findOne({category: "question"}))!.id)

        if (questionChannel?.type != ChannelType.GuildText)
            return;

        const text = interaction.options.getString("question", true)

        const question: IQuestion = {
            text: text,
            userId: interaction.member.id,
            questionId: await questionsCollection.count(),
            messageId: undefined,
            answer: undefined,
        }

        const embed = new Discord.EmbedBuilder()
            .addFields([
                {name: `Question id: ${question.questionId}`, value: text}
            ])

        const msg = await questionChannel.send({embeds: [embed]})

        question.messageId = msg.id
        await (await questionsCollection.create(question)).save()
    }
}
