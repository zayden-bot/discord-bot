import Discord from "discord.js"
import {getServer} from "../../../models/server";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("saves")
        .setDescription("Get saves disclaimer"),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const server = await getServer(interaction.guild.id)

        interaction.reply(`We do our best to retain save integrity with every update however due to the dynamic nature of game development saves might break. If you experience a save problem please let us know in <#${server.channels.supportChannel}>\n\nReminder:\nWith the major changes in v0.6 saves before this version will not work. We are sorry for the inconvenience. Use CTRL/TAB to quickly skip through the content you have seen`).then()
    },
}
