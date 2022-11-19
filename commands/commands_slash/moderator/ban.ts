import Discord from "discord.js"
import {addLog, LogType} from "./functions";
import {getServer} from "../../../models/server";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("ban")
        .setDescription("Ban a member")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.BanMembers)
        .addUserOption(option =>
            option.setName("member")
                .setDescription("Member to ban")
                .setRequired(true))
        .addStringOption(option =>
            option.setName("reason")
                .setDescription("Reason for the ban")),

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

        const serverMsg = new Discord.EmbedBuilder()
            .setTitle("User Banned")
            .setDescription(`${member} has been banned by College Kings Staff`)
            .setColor("#ff0000")

        const privateMsg = new Discord.EmbedBuilder()
            .setDescription(`You were banned in ${interaction.guild.name} for:\n${reason}`)

        await addLog(server, LogType.Ban, interaction.guild, member, interaction.user, reason)

        Promise.all([
            member.ban({deleteMessageDays: 7, reason: reason}),
            interaction.reply({embeds: [serverMsg]}),
            member.user.send({embeds: [privateMsg]})
        ]).catch(() => {
            interaction.reply({content: `Failed to ban ${member.user.username}`, ephemeral: true})
        })
    }
}
