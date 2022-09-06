import Discord from "discord.js"
import {ChannelType} from "discord-api-types/v10"

module.exports = {
    commands: ["serverinfo"],
    minArgs: 0,
    maxArgs: 0,
    callback: (message: Discord.Message) => {
        if (!message.guild) {
            return;
        }

        const icon = message.guild.iconURL() as string;

        const embed = new Discord.EmbedBuilder()
            .setAuthor({name: message.guild.name, iconURL: icon})
            .addFields([
                {name: "Owner", value: `<@${message.guild.ownerId}>`, inline: true},
                {
                    name: "Channel Categories",
                    value: message.guild.channels.cache.filter(channel => channel.type === ChannelType.GuildCategory).size.toString(),
                    inline: true
                },
                {
                    name: "Text Channels",
                    value: message.guild.channels.cache.filter(channel => channel.type === ChannelType.GuildText).size.toString(),
                    inline: true
                },
                {
                    name: "Voice Channels",
                    value: message.guild.channels.cache.filter(channel => channel.type === ChannelType.GuildVoice).size.toString(),
                    inline: true
                },
                {name: "Members", value: message.guild.memberCount.toString(), inline: true},
                {name: "Roles", value: message.guild.roles.cache.size.toString(), inline: true}
            ])
            .setFooter({text: `ID: ${message.guild.id} | Server Created: ${message.guild.createdAt.getFullYear()}-${message.guild.createdAt.getMonth()}-${message.guild.createdAt.getDate()}`})
            .setThumbnail(icon)

        message.channel.send({embeds: [embed]})
    },
}
