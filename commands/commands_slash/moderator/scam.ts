import Discord from "discord.js"
import {addLog, LogType} from "./functions";
import {getServer} from "../../../models/server";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("scam")
        .setDescription("Soft bans a compromised account sending scam links")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.KickMembers)
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
        const reason = interaction.options.getString("reason") ?? "Compromised account: Sending scam links."

        if (!(member instanceof Discord.GuildMember)) {
            return interaction.reply({content: "Invalid member mention", ephemeral: true})
        }

        const response = new Discord.EmbedBuilder()
            .setTitle("Scammer Soft banned")
            .setDescription(`${member} has been successfully soft banned`)
            .setColor("#ff0000")

        const privateMsg = new Discord.EmbedBuilder()
            .setDescription(`You were soft banned in ${interaction.guild.name} for:\n${reason}`)

        await addLog(server, LogType.SoftBan, interaction.guild, member, interaction.user, reason)

        member.user.send({embeds: [privateMsg]}).catch()

        member.ban({
            deleteMessageSeconds: 604800,
            reason: reason
        }).catch(() => interaction.reply({content: `Failed to ban ${member}`, ephemeral: true}))

        await interaction.reply({embeds: [response], ephemeral: true})

        interaction.guild.members.unban(member).catch(async () => await interaction.editReply({
            content: `Failed to unban ${member}`
        }))

    }
}
