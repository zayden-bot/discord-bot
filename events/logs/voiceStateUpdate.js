const { MessageEmbed } = require("discord.js");
const config = require("../../Configs/ckConfig.json");

module.exports = {
    event: "voiceStateUpdate",
    callback: (...args) => {
        const oldState = args[0][0];
        const newState = args[0][1];

        // if (newState && newState.channelID) {
        //     const embed = new MessageEmbed()
        //         .setTitle('Member connected to a voice chat!')
        //         .addField("Member", `<@${newState.member.id}>`, true)
        //         .addField("Channel", `${newState.channel.name}`, true)
        //         .setColor("#00ff00")
        //         .setTimestamp();

        //         newState.guild.channels.cache.get(config.logsChannel).send(embed);
        // } else {
        //     const embed = new MessageEmbed()
        //         .setTitle('Member disconnected to a voice chat!')
        //         .addField("Member", `<@${oldState.member.id}>`, true)
        //         .addField("Channel", `${oldState.channel.name}`, true)
        //         .setColor('#ff0000')
        //         .setTimestamp();
                

        //     oldState.guild.channels.cache.get(config.logsChannel).send(embed);
        // }
    },
}
