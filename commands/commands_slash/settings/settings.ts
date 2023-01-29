import Discord from "discord.js";
import {add_support_channel} from "./add_support_channel";
import {getConnection} from "../../../servers";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("settings")
        .setDescription("Modify the bot inside your Discord server")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.Administrator)
        .addSubcommand(command =>
            command.setName("add_support_channel")
                .setDescription("Adds a channel to the support channel list")
                .addChannelOption(option =>
                    option.setName("channel")
                        .setDescription("Channel to add to channel list")
                        .setRequired(true))),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const conn = getConnection(interaction.guild.id)

        const subcommand = interaction.options.getSubcommand()

        switch (subcommand) {
            case "add_support_channel":
                const channel = interaction.options.getChannel("channel", true);
                if (channel instanceof Discord.TextChannel)
                    await add_support_channel(interaction, conn, channel)
                break;
            default:
                await interaction.reply("Invalid subcommand selected")
                break
        }
    }
}
