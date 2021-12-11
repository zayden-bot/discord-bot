import Discord from "discord.js"

module.exports = {
    commands: ["suggest", "suggestion"],
    expectedArgs: "<text>",
    permissionError: "",
    minArgs: 1,
    callback: (message: Discord.Message, args: string[], text: string) => {
        if (!message.guild) { return; }

        const config = require(`../../server_configs/${message.guild.id}.json`);

        const embed = new Discord.MessageEmbed()
        .setTitle(`From: ${message.author.username}`)
        .setDescription(text);

        let channel = message.guild.channels.cache.get(config.channels.suggestionChannel);
        if (channel && channel.isText()) {
            channel.send({embeds: [embed]})
            .then((message: Discord.Message) => {
                message.react("👍");
                message.react("👎");
            })
        }
    },
}
