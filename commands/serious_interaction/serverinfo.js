const Discord = require("discord.js")

module.exports = {
    commands: ["serverinfo"],
    minArgs: 0,
    maxArgs: 0,
    callback: (message, arguments, text) => {
        

        let roles = []
        message.guild.roles.cache.forEach(role => roles.push(role.name))

        const embed = new Discord.MessageEmbed()
        .setAuthor(message.guild.name, message.guild.iconURL())
        .addFields(
            { name: "Owner", value: `<@${message.guild.ownerID}>`, inline: true },
            { name: "Region", value: message.guild.region.charAt(0).toUpperCase() + message.guild.region.slice(1), inline: true },
            { name: "Channel Categories", value: message.guild.channels.cache.filter(channel => channel.type === "category").size, inline: true },
            { name: "Text Channels", value: message.guild.channels.cache.filter(channel => channel.type === "text").size, inline: true },
            { name: "Voice Channels", value: message.guild.channels.cache.filter(channel => channel.type === "voice").size, inline: true },
            { name: "Members", value: message.guild.memberCount, inline: true },
        )
        .addField("Roles", message.guild.roles.cache.size)
        .addField("Role List", roles.join(", "))
        .setFooter(`ID: ${message.guild.id} | Server Created: ${message.guild.createdAt.getFullYear()}-${message.guild.createdAt.getMonth()}-${message.guild.createdAt.getDate()}`)
        .setThumbnail(message.guild.iconURL())

        message.channel.send(embed)
    },
}
