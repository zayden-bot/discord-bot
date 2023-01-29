import Discord from "discord.js"
import {getConnection} from "../../../servers";
import {IChannel} from "../../../models/server_settings/ChannelSchema";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("saves")
        .setDescription("Get saves disclaimer"),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const conn = getConnection(interaction.guild.id)
        const supportChannel = await conn.model<IChannel>("Channels").findOne({category: "support"})
        if (!supportChannel)
            return

        await interaction.reply(`We do our best to retain save integrity with every update however due to the dynamic nature of game development saves might break. If you experience a save problem please let us know in <#${supportChannel.id}>`)
    }
}
