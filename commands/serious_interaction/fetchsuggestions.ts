import Discord from "discord.js";
import {IServer} from "../../models/server";

module.exports = {
    commands: ["fetchsuggestions"],
    permissionError: "",
    callback: async (message: Discord.Message, server: IServer) => {
        const guild = message.guild;
        if (!guild) {
            return;
        }

        const startTime = new Date();
        const statusMessage = await message.channel.send("Fetching information...");
        const suggestionChannel = await guild.channels.fetch(server.channels.suggestionChannel)
        if (!suggestionChannel || !suggestionChannel.isText()) {
            return message.reply("Invalid suggestion channel");
        }

        const suggestionMessages = (await suggestionChannel.messages.fetch({limit: 100}))
            .filter(message => message.author.id === '787490197943091211' && message.embeds.length > 0)
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

        let index = 0
        let count = 0;
        let embed = new Discord.MessageEmbed()
            .setColor("#ff0000")
            .setTitle(`Top ${suggestionMessages.size} suggestions!`)
            .setDescription(`Here are the top ${suggestionMessages.size} suggestions, ordered by UP-Votes, excluding the Bot Vote!`);

        for (const [, element] of suggestionMessages) {
            const thumbsUp = element.reactions.resolve('👍')
            const thumbsDown = element.reactions.resolve('👎')
            index++

            if (!thumbsUp || !thumbsDown) {
                continue
            }

            embed.addField(
                `Position: ${index}, 👍: ${thumbsUp.count - 1}, 👎: ${thumbsDown.count - 1}`,
                `Link: https://discord.com/channels/${guild.id}/${server.channels.suggestionChannel}/${element.id}`,
                false
            );

            count++

            if (index == suggestionMessages.size) {
                embed.setImage("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg?width=1440&height=360");
                embed.setTimestamp();

                await message.author.send({embeds: [embed]});
            } else if (count > 20) {
                await message.author.send({embeds: [embed]});

                count = 0
                embed = new Discord.MessageEmbed()
                    .setColor("#ff0000")
                    .setThumbnail("");
            }
        }

        const endTime = new Date();

        statusMessage.edit(`Information sent in DMs, elapsed time: ${Math.round((endTime.getTime() - startTime.getTime()) / 1000)} second(s)!`);
    },
    permissions: ["MANAGE_MESSAGES"],
}
