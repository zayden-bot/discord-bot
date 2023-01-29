import Discord from "discord.js"
import {addLog, ban, LogType, mute, warn} from "./functions";
import {getUserConfig} from "../../../models/user-config";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("infraction")
        .setDescription("Adds an infraction a user")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.ModerateMembers)
        .addUserOption(option =>
            option.setName("member")
                .setDescription("Member to add infraction to")
                .setRequired(true))
        .addIntegerOption(option =>
            option.setName("points")
                .setDescription("Number of infraction points to give a user")
                .setMinValue(0)
                .setMaxValue(5))
        .addStringOption(option =>
            option.setName("reason")
                .setDescription("Reason for the ban")),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }
        
        const member = interaction.options.getMember("member")
        const reason = interaction.options.getString("reason") ?? "No reason given"
        const infractionPoints = interaction.options.getInteger("points") ?? 1

        if (!(member instanceof Discord.GuildMember)) {
            return interaction.reply({content: "Invalid member mention", ephemeral: true})
        }

        const user = await getUserConfig(member.id)

        user.infractions = Math.min(infractionPoints + user.infractions, 5)
        switch (user.infractions) {
            case 1:
                await warn(interaction.guild, interaction.channel!, member, interaction.user, reason);
                break;
            case 2:
                await mute(interaction.guild, interaction.channel!, member, interaction.user, 3600000, reason)
                break;
            case 3:
                await mute(interaction.guild, interaction.channel!, member, interaction.user, 28800000, reason)
                break;
            case 4:
                await mute(interaction.guild, interaction.channel!, member, interaction.user, 604800000, reason)
                break;
            case 5:
                await ban(interaction.guild, interaction.channel!, member, interaction.user, reason)
                break;
            default:
                break;
        }

        const response = new Discord.EmbedBuilder()
            .setTitle("Added Infraction")
            .setDescription(`${member} has been given ${infractionPoints} infraction points`)
            .setColor("#ff0000")

        await Promise.all([
            addLog(interaction.guild.id, LogType.Infraction, member, interaction.user.id, reason),
            interaction.reply({embeds: [response], ephemeral: true})
        ])
    }
}
