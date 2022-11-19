import {getServer} from "../../../models/server";
import Discord from "discord.js";
import {addLog, LogType} from "./functions";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("warn")
        .setDescription("warn a member")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.ManageMessages)
        .addUserOption(option =>
            option.setName("member")
                .setDescription("Member to warn")
                .setRequired(true))
        .addStringOption(option =>
            option.setName("reason")
                .setDescription("Reason for the warning")),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const server = await getServer(interaction.guild.id)
        const member = interaction.options.getMember("member")
        const reason = interaction.options.getString("reason") || "No Reason Given"

        if (!(member instanceof Discord.GuildMember)) {
            return interaction.reply({content: "Invalid member mention", ephemeral: true})
        }

        await addLog(server, LogType.Warn, interaction.guild, member, interaction.user, reason)

        const serverMsg = new Discord.EmbedBuilder()
            .setTitle(`User Warned`)
            .setDescription(`**${member} has been warned by ${interaction.user}\nReason: ${reason}**`)
            .setColor("#ff0000")

        const privateMsg = new Discord.EmbedBuilder()
            .setDescription(`You were warned in ${interaction.guild.name} for: ${reason}`)

        await Promise.all([
            interaction.reply({embeds: [serverMsg]}),
            member.user.send({embeds: [privateMsg]})
        ])
    }
}
