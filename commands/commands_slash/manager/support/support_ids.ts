import Discord from "discord.js";
import {getServer} from "../../../../models/server";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("support_ids")
        .setDescription("Get a list of valid support IDs")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.MoveMembers),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const server = await getServer(interaction.guild.id)

        const ids = Array.from(server.supportAnswers.keys())

        if (ids.length == 0) {
            return interaction.reply({content: "No support ids for this server", ephemeral: true})
        }

        await interaction.reply(`\`\`\`${ids.sort().join("\n")}\`\`\``)
    },
    requiredRoles: ["Support Team"]
}
