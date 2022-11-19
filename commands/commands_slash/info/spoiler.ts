import Discord from "discord.js"
import {getServer} from "../../../models/server";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("spoilers")
        .setDescription("Disclaimer about spoilers"),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const server = await getServer(interaction.guild.id)

        interaction.reply(`Please keep all conversations about the new update to <#770621445637799946>\nIf you have any bugs or questions please post them in <#${server.channels.supportChannel}>`).then()
    },
}
