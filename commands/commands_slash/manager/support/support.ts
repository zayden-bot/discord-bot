import Discord from "discord.js";
import {getServer} from "../../../../models/server";


module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("support")
        .setDescription("Send support information")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.MoveMembers)
        .addStringOption(option =>
            option.setName("id")
                .setDescription("Enter support ID")
                .setRequired(true)),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const server = await getServer(interaction.guild.id)

        const id = interaction.options.getString("id", true).toLowerCase();
        const answer = server.supportAnswers.get(id);
        if (!answer) {
            return interaction.reply({content: `There is no support answer for ID: ${id}`, ephemeral: true});
        }

        const embed = new Discord.EmbedBuilder()
            .setTitle(`Support ID: ${id[0].toUpperCase() + id.slice(1)}`)
            .setDescription(answer)
            .setColor("#ff0000")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png");

        interaction.reply({embeds: [embed]}).then()
    },
}
