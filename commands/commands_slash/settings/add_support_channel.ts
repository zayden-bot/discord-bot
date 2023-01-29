import Discord from "discord.js";
import mongoose from "mongoose";
import {IChannel} from "../../../models/server_settings/ChannelSchema";

export async function add_support_channel(interaction: Discord.ChatInputCommandInteraction, conn: mongoose.Connection, channel: Discord.TextChannel) {
    const channelsCollection = await conn.model<IChannel>("Channels")

    await (await channelsCollection.create({
        category: "support",
        name: channel.name,
        type: channel.type,
        id: channel.id
    })).save()

    await interaction.reply({content: "Successfully added support channel", ephemeral: true})
}
