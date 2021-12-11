import Discord from "discord.js"

module.exports = {
    commands: ["serverinfo"],
    minArgs: 0,
    maxArgs: 0,
    callback: (message: Discord.Message, args: string[], text: string) => {
        if (!message.guild) { return; }

        const icon = message.guild.iconURL() as string;

        const embed = new Discord.MessageEmbed()
        .setAuthor(message.guild.name, icon)
        .addField("Owner", `<@${message.guild.ownerId}>`, true)
        .addField("Channel Categories", message.guild.channels.cache.filter(channel => channel.type === "GUILD_CATEGORY").size.toString(), true)
        .addField("Text Channels", message.guild.channels.cache.filter(channel => channel.type === "GUILD_TEXT").size.toString(), true)
        .addField("Voice Channels", message.guild.channels.cache.filter(channel => channel.type === "GUILD_VOICE").size.toString(), true)
        .addField("Members", message.guild.memberCount.toString(), true)
        .addField("Roles", message.guild.roles.cache.size.toString(), true)
        .setFooter(`ID: ${message.guild.id} | Server Created: ${message.guild.createdAt.getFullYear()}-${message.guild.createdAt.getMonth()}-${message.guild.createdAt.getDate()}`)
        .setThumbnail(icon)

        message.channel.send({embeds: [embed]})
    },
}
