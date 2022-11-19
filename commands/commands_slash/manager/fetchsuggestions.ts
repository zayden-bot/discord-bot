import Discord from "discord.js";
import {ChannelType} from "discord-api-types/v10"
import {getServer} from "../../../models/server";

module.exports = {
    data: new Discord.SlashCommandBuilder()
        .setName("fetch_suggestions")
        .setDescription("Fetch top community suggestions")
        .setDefaultMemberPermissions(Discord.PermissionFlagsBits.ManageMessages),

    async execute(interaction: Discord.ChatInputCommandInteraction) {
        if (!interaction.guild) {
            return;
        }

        const server = await getServer(interaction.guild.id)

        const startTime = new Date();
        interaction.reply({content: "Fetching information...", ephemeral: true}).then();
        const suggestionChannel = await interaction.guild.channels.fetch(server.channels.suggestionChannel)
        if (!suggestionChannel || suggestionChannel.type != ChannelType.GuildText) {
            return interaction.editReply({content: "Invalid suggestion channel"});
        }

        let suggestionMessages: Discord.Collection<string, Discord.Message> = new Discord.Collection();
        let lastMessage;
        let previousCollectionSize = -1;
        let currentCollectionSize = suggestionMessages.size
        while (previousCollectionSize != currentCollectionSize) {
            previousCollectionSize = currentCollectionSize
            suggestionMessages = suggestionMessages.concat((await suggestionChannel.messages.fetch({
                limit: 100,
                before: lastMessage?.id
            })))
            lastMessage = suggestionMessages.last()
            currentCollectionSize = suggestionMessages.size
        }
        suggestionMessages = suggestionMessages
            .filter(message => message.author.id == message.client.user?.id && message.embeds.length > 0)
            .sort((a, b) => {
                const a_thumbsUp = a.reactions.resolve('👍')
                const b_thumbsUp = b.reactions.resolve('👍')
                if (!a_thumbsUp) {
                    return -1
                }
                if (!b_thumbsUp) {
                    return 1
                }

                if (a_thumbsUp.count - 1 > b_thumbsUp.count - 1) {
                    return -1;
                } else if (a_thumbsUp.count - 1 < b_thumbsUp.count - 1) {
                    return 1;
                } else {
                    return 0;
                }
            })

        let count = 1;
        let embed = new Discord.EmbedBuilder()
            .setColor("#ff0000")
            .setTitle(`Top 10 suggestions!`)
            .setDescription(`Here are the top 10 suggestions, ordered by Votes`);

        for (const [, element] of suggestionMessages) {
            const thumbsUp = element.reactions.resolve('👍')
            const thumbsDown = element.reactions.resolve('👎')

            if (!thumbsUp || !thumbsDown) {
                continue
            }

            embed.addFields([
                {
                    name: `Position: ${count}, 👍: ${thumbsUp.count - 1}, 👎: ${thumbsDown.count - 1}`,
                    value: `Link: https://discord.com/channels/${interaction.guild.id}/${server.channels.suggestionChannel}/${element.id}`,
                    inline: false
                }
            ]);

            if (count == 10) {
                embed.setImage("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg?width=1440&height=360");
                embed.setTimestamp();

                await interaction.user.send({embeds: [embed]});
                break;
            }
            count++
        }

        const endTime = new Date();

        interaction.editReply(`Sent the top 10 suggestions out of ${suggestionMessages.size}, elapsed time: ${Math.round((endTime.getTime() - startTime.getTime()) / 1000)} second(s)!`).then();
    },
}
