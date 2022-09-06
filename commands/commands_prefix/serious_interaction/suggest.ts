import Discord from "discord.js"
import {IServer} from "../../../models/server";
import {ChannelType} from "discord-api-types/v10"

module.exports = {
    commands: ["suggest", "suggestion"],
    expectedArgs: "<text>",
    callback: async (message: Discord.Message, server: IServer, args: string[], text: string) => {
        if (!message.guild) {
            return;
        }

        const embed = new Discord.EmbedBuilder()
            .setTitle(`From: ${message.author.username}`)
            .setDescription(text);

        let channel = await message.guild.channels.fetch(server.channels.suggestionChannel);
        if (channel && channel.type == ChannelType.GuildText) {
            message = await channel.send({embeds: [embed]})
            await message.react("👍");
            await message.react("👎");
        }
    },
}
