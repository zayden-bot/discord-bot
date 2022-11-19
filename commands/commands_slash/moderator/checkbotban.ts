import {isBlacklisted} from "./functions";
import Discord from "discord.js";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("checkbotban")
        .setDescription("Check the bot ban status of a member")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.ManageMessages)
        .addUserOption(option =>
            option.setName("member")
                .setDescription("Member to check bot ban status")
                .setRequired(true)),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        const member = interaction.options.getMember("member")

        if (!(member instanceof Discord.GuildMember)) {
            return interaction.reply({content: "Invalid member mention", ephemeral: true})
        }

        if (await isBlacklisted(member)) {
            await interaction.reply("The user is blacklisted!");
        } else {
            await interaction.reply("The user is not blacklisted!");
        }
    }
}
