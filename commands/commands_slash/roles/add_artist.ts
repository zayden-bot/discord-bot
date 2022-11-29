import Discord from "discord.js";


module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("add_artist")
        .setDescription("Grant a member the artist role")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.ManageMessages)
        .addUserOption(option =>
            option.setName("member")
                .setDescription("Member to grant the Artist role too")
                .setRequired(true)),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const member = interaction.options.getMember("member")
        if (!(member instanceof Discord.GuildMember)) {
            return interaction.reply({content: "Invalid member mention", ephemeral: true})
        }

        let artistRole = (interaction.guild.roles.cache.find(role => role.name == "Artist") || await interaction.guild.roles.create({
            name: "Artist"
        }));

        if (!artistRole) {
            return interaction.reply({content: "Failed to find or create Artist role", ephemeral: true});
        }

        await member.roles.add(artistRole)
        interaction.reply(`${member} was given the Artist role`).then()
    },
}
