import Discord from "discord.js";
import {getConnection} from "../../../../servers";
import {ISupportFAQ} from "../../../../models/server_settings/SupportFAQSchema";


module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("support_ids")
        .setDescription("Get a list of valid support IDs")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.MoveMembers),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const conn = getConnection(interaction.guild.id)
        const supportIds = (await conn.model<ISupportFAQ>("SupportFAQ").find().lean()).map(support => support._id)

        if (supportIds.length == 0) {
            return interaction.reply({content: "No support ids for this server", ephemeral: true})
        }

        await interaction.reply(`\`\`\`${supportIds.sort().join("\n")}\`\`\``)
    }
}
