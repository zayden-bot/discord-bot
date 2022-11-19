import {getServer} from "../../../models/server";
import Discord from "discord.js";
import {LogType} from "./functions";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("clearwarnings")
        .setDescription("Clear the warnings from a member")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.ManageMessages)
        .addUserOption(option =>
            option.setName("member")
                .setDescription("Member to clear all warnings from")
                .setRequired(true)),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const server = await getServer(interaction.guild.id)
        const member = interaction.options.getMember("member")

        if (!(member instanceof Discord.GuildMember)) {
            return interaction.reply({content: "Invalid member mention", ephemeral: true})
        }

        const startLength = server.moderation.length

        server.moderation = server.moderation.filter((log) => !(log.userId == member.id && log.logType == LogType.Warn))

        await Promise.all([
            server.save(),
            interaction.reply(`Cleared ${startLength - server.moderation.length} warnings from ${member.user.username}`)
        ])
    },
    permissions: ["MANAGE_MESSAGES"],
}
