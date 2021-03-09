const Discord = require("discord.js")
const config = require("../../serverConfigs/CKConfig.json")

module.exports = {
    commands: ["fetchSuggestions", "fetchsuggestions"],
    permissionError: "",
    requiredRoles: ["Security"],
    callback: async (message) => {
        var startTime, endTime;
        startTime = new Date();
        const statusMessage = await message.channel.send("Fetching information...");
        message.guild.channels.cache.get(config.suggestionChannel).messages.fetch({limit: 100})
            .then(async messages => {
                const botMessages = messages // Filtering and sorting
                        .filter(c => c.author.id === '787490197943091211' && c.embeds.length > 0 && c.reactions.resolve('👍') && c.reactions.resolve('👎'))
                        .sort((a, b) => {
                        if (a.reactions.resolve('👍').count-1 > b.reactions.resolve('👍').count-1) {
                            return -1;
                        }
                        if (a.reactions.resolve('👍').count-1 < b.reactions.resolve('👍').count-1) {
                            return 1;
                        }
                        return 0;
                        });
                const messageCount = botMessages.size;
                var embedCount = Math.ceil(messageCount/20);

                for (i = 0; i < embedCount; i++) {
                    var embed = new Discord.MessageEmbed();
                    embed.setColor("ff0000");
                    embed.setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566");
                    
                    var index = 0;
                    const startIndex = (0+(20*i));
                    const endIndex = (0+(20*i))+20;
                    botMessages.forEach((element) => {
                        index++;
                        if (index >= startIndex-1 && index <= endIndex-1) {
                            embed.addField(`Position: ${index}, 👍: ${element.reactions.resolve('👍').count-1}, 👎: ${element.reactions.resolve('👎').count-1}`,`Link: https://discord.com/channels/${message.guild.id}/${config.suggestionChannel}/${element.id}`,false);
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

                    await message.author.send(embed);
                }
                endTime = new Date();
                var timeDiff = endTime - startTime;
                statusMessage.edit(`Information sent in DMs, elapsed time: ${Math.round(timeDiff /= 1000)} second(s)!`);
            });
    },
}