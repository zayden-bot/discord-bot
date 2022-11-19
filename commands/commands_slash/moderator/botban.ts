import Discord from "discord.js";
import {BotConfig, IBotConfig} from "../../../models/bot-config";
import {LogType} from "./functions";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("botban")
        .setDescription("Bot ban a member")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.ManageMessages)
        .addUserOption(option =>
            option.setName("member")
                .setDescription("Member to ban")
                .setRequired(true))
        .addStringOption(option =>
            option.setName("reason")
                .setDescription("Reason for the ban")
                .setRequired(true)),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const member = interaction.options.getMember("member")
        const reason = interaction.options.getString("reason", true)

        if (!(member instanceof Discord.GuildMember)) {
            return interaction.reply({content: "Invalid member mention", ephemeral: true})
        }

        const botConfigJson = require("../../../configs/bot_config.json")
        if (botConfigJson.developers.includes(member.id)) {
            return interaction.reply("This is a protected member and cannot be bot banned");
        }

        const botConfig: IBotConfig | null = await BotConfig.findOne<IBotConfig>().exec()
        const botBans = new Set(botConfig!.botBan)
            .add({
                caseNumber: botConfig!.botBan.length,
                userId: member.id,
                logType: LogType.BotBan.toString(),
                moderatorId: interaction.user.id,
                reason: reason
            })
        botConfig!.botBan = Array.from(botBans)

        await Promise.all([
            botConfig!.save(),
            interaction.reply(`Successfully bot banned ${member}`)
        ])
    }
}
