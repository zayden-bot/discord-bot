import Discord from "discord.js"
import {getServer, IQuestion, IServer} from "../../../../models/server";
import {ChannelType} from "discord-api-types/v10"

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

        const server: IServer = await getServer(interaction.guild.id)

        const questionChannel = await interaction.guild.channels.fetch(server.channels.questionChannel)
        if (!questionChannel || questionChannel.type != ChannelType.GuildText) {
            return;
        }

        const text = interaction.options.getString("question") || ""

        const question: IQuestion = {
            text: text,
            userId: interaction.member.id,
            questionId: server.questions.length,
            messageId: undefined,
            answer: undefined,
        }

        const embed = new Discord.EmbedBuilder()
            .addFields([
                {name: `Question id: ${question.questionId}`, value: text}
            ])

        const msg = await questionChannel.send({embeds: [embed]})
        question.messageId = msg.id
        server.questions.push(question)
        server.save().then()
    }
}
