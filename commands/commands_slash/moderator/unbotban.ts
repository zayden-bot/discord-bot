import Discord from "discord.js";
import {BotConfig, IBotConfig} from "../../../models/bot-config";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("botban")
        .setDescription("Unbotban a member")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.ManageMessages)
        .addUserOption(option =>
            option.setName("member")
                .setDescription("Member to unbotban")
                .setRequired(true))
        .addStringOption(option =>
            option.setName("reason")
                .setDescription("Reason for the unbotban")),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const member = interaction.options.getMember("member")

        if (!(member instanceof Discord.GuildMember)) {
            return interaction.reply({content: "Invalid member mention", ephemeral: true})
        }

        const botConfig: IBotConfig | null = await BotConfig.findOne<IBotConfig>().exec()
        botConfig!.botBan = botConfig!.botBan.filter((banLog) => banLog.userId != member.id)

        await Promise.all([
            botConfig!.save(),
            interaction.reply(`Successfully Removed Bot Ban from ${member}`)
        ])
    }
}
