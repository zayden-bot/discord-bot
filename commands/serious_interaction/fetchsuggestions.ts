import Discord from "discord.js";

module.exports = {
    commands: ["fetchSuggestions", "fetchsuggestions"],
    permissionError: "",
    callback: async (message: Discord.Message, args: string[], text: string) => {
        const guild = message.guild;
        if (!guild) { return; }

        const config = require("../../server_configs/745662812335898806.json")

        const startTime = new Date();
        const statusMessage = await message.channel.send("Fetching information...");
        const suggestionChannel = await guild.channels.fetch(config.suggestionChannel)
        if (!suggestionChannel || !suggestionChannel.isText()) { return message.reply("Invalid suggestion channel"); }

        suggestionChannel.messages.fetch({limit: 100})
            .then(messages => {
                const botMessages = messages // Filtering and sorting
                        .filter(message => message.author.id === '787490197943091211' && message.embeds.length > 0)
                        .sort((a, b) => {
                            const a_thumbsUp = a.reactions.resolve('👍')
                            const b_thumbsUp = b.reactions.resolve('👍')
                            if (!a_thumbsUp) { return -1 }
                            if (!b_thumbsUp) { return 1 }

                            if (a_thumbsUp.count-1 > b_thumbsUp.count-1) { return -1; }
                            else if (a_thumbsUp.count-1 < b_thumbsUp.count-1) { return 1; }
                            else { return 0; }
                        });
                const messageCount = botMessages.size;
                var embedCount = Math.ceil(messageCount/20);

                for (let i = 0; i < embedCount; i++) {
                    var embed = new Discord.MessageEmbed();
                    embed.setColor("#ff0000");
                    embed.setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566");
                    
                    var index = 0;
                    const startIndex = (0+(20*i));
                    const endIndex = (0+(20*i))+20;
                    botMessages.forEach((element) => {
                        index++;

                        const reaction = element.reactions.resolve('👍')
                        if (!reaction) { return; }

                        if (index >= startIndex-1 && index <= endIndex-1) {
                            embed.addField(`Position: ${index}, 👍: ${reaction.count-1}, 👎: ${reaction.count-1}`,`Link: https://discord.com/channels/${guild.id}/${config.suggestionChannel}/${element.id}`,false);
                        }
                    })

                    if (i === 0) {
                        embed.setTitle(`Top ${messageCount} suggestions!`);
                        embed.setDescription(`Here are the top ${messageCount} suggestions, ordered by UP-Votes, excluding the Bot Vote!`);
                    }

                    if (i === (embedCount-1)) {
                        embed.setImage("https://media.discordapp.net/attachments/769943204673486858/787791290514538516/CollegeKingsTopBanner.jpg?width=1440&height=360");
                        embed.setTimestamp();
                    }

                    message.author.send({embeds: [embed]});
                }
                
                const endTime = new Date();
                statusMessage.edit(`Information sent in DMs, elapsed time: ${Math.round(endTime.getTime() - startTime.getTime() / 1000)} second(s)!`);
            });
    },
    permissions: ["MANAGE_MESSAGES"],
}
