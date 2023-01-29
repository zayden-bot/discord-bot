import Discord from "discord.js"
import {getConnection} from "../../../servers";
import {IChannel} from "../../../models/server_settings/ChannelSchema";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("spoilers")
        .setDescription("Disclaimer about spoilers"),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const conn = getConnection(interaction.guild.id)
        const supportChannel = await conn.model<IChannel>("Channels").findOne({category: "support"})
        const spoilerChannel = await conn.model<IChannel>("Channels").findOne({category: "spoiler"})
        if (!supportChannel || !spoilerChannel)
            return

        await interaction.reply(`Please keep all conversations about the new update to <#${spoilerChannel.id}>\nIf you have any bugs or questions please post them in <#${supportChannel.id}>`)
    },
}
