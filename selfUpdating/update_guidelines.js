const Discord = require("discord.js");

module.exports = async (client, channelId) => {
    const server_config = require("../serverConfigs/745662812335898806.json")
    const channel = await client.channels.fetch(channelId)

    channel.bulkDelete(100)

    for (guideline of server_config["server_guidelines"]) {
        const guideline_msg = new Discord.MessageEmbed()
            .setTitle(guideline[0])
            .setDescription(guideline[1])
            .addField("Action", guideline[2])
            .setColor("ff0000")
        channel.send(guideline_msg)
    }
}
