import Discord from "discord.js";
import {addLog, LogType} from "./functions";
import {getServer} from "../../../models/server";


module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("mute")
        .setDescription("Mute a member so they cannot type")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.ManageMessages)
        .addUserOption(option =>
            option.setName("member")
                .setDescription("Member to mute")
                .setRequired(true))
        .addStringOption(option =>
            option.setName("duration")
                .setDescription("How long to mute the user for. Ex: 5m, 3h, 1d"))
        .addStringOption(option =>
            option.setName("reason")
                .setDescription("Reason for the mute")),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const server = await getServer(interaction.guild.id)
        const member = interaction.options.getMember("member")
        const duration_string = interaction.options.getString("duration") || ""
        const reason = interaction.options.getString("reason") || "No Reason Given"

        if (!(member instanceof Discord.GuildMember)) {
            return interaction.reply({content: "Invalid member mention", ephemeral: true})
        }

        let duration = -1
        if (duration_string) {
            const magnitude = Number(duration_string.slice(0, -1))
            const delimiter = duration_string.slice(-1)

            const durations: Record<string, number> = {
                's': 1,
                'm': 60,
                'h': 60 * 60,
                'd': 60 * 60 * 24
            }

            duration = magnitude * durations[delimiter]
        }

        let mutedRole = (interaction.guild.roles.cache.find(role => role.name == "Muted") || await interaction.guild.roles.create({
            name: "Muted",
            color: "#818386"
        }));

        if (!mutedRole) {
            return interaction.reply({content: "Failed to create muted role", ephemeral: true});
        }

        interaction.guild.channels.cache.forEach((channel) => {
            if (!(channel instanceof Discord.ThreadChannel)) {
                channel.permissionOverwrites.create(mutedRole, {
                    SendMessages: false,
                    Speak: false,
                    AddReactions: false
                })
            }
        })

        const serverMsg = new Discord.EmbedBuilder()
            .setTitle(`User Muted`)
            .setDescription(`<@${member.id}> has been muted by CK Staff for: ${reason}`)
            .setColor("#ff0000")

        const privateMsg = new Discord.EmbedBuilder()
            .setDescription(`You were muted in ${interaction.guild.name} for: ${reason}`)

        await addLog(server, LogType.Mute, interaction.guild, member, interaction.user, reason)

        await member.roles.add(mutedRole)

        await Promise.all([
            interaction.reply({embeds: [serverMsg]}),
            member.user.send({embeds: [privateMsg]})
        ])

        if (duration > 0) {
            setTimeout(() => {
                member.roles.remove(mutedRole);
            }, duration * 1000)
        }
    }
}
