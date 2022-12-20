import Discord from "discord.js";
import {getServer} from "../../../../models/server";


module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("support_channel_add")
        .setDescription("Add a channel to the support channels list")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.Administrator)
        .addChannelOption(option =>
            option.setName("channel")
                .setDescription("Enter support channel")
                .setRequired(true)),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const channel = interaction.options.getChannel("channel", true);

        const server = await getServer(interaction.guild.id)
        server.channels.supportChannels.push(channel.id)
        server.save().then()

        interaction.reply({content: "Successfully added support channel", ephemeral: true}).then()
    },
}
