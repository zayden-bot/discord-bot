import Discord from "discord.js";

module.exports = async function (client: Discord.Client, channelId: string) {
    const server_config = require("../server_configs/745662812335898806.json")
    const channel = await client.channels.fetch(channelId) as Discord.TextChannel
    if (!channel || channel.type !== "GUILD_TEXT" ) { return console.error("Invalid channel id") }

    channel.bulkDelete(100)

    for (const guideline of server_config["server_guidelines"]) {
        const guideline_msg = new Discord.MessageEmbed()
            .setTitle(guideline[0])
            .setDescription(guideline[1])
            .addField("Action", guideline[2])
            .setColor("#ff0000")
        channel.send({embeds: [guideline_msg]});
    }
}
