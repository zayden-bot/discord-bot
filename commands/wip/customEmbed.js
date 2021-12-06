const Discord = require("discord.js");

module.exports = {
    commands: ["custom", "cm"],
    callback: (message, arguments, text) => {
        const embed = new Discord.MessageEmbed()
            .setTitle(`Join a CK Custom Club!`)
            .setDescription(`
            :one: <@&805766527889440778>

            :two: <@&805766587704803339>

            :three: <@&805766682202079273>

            :four: <@&805766763215847454>

            :five: <@&805766896233742346>
            `)
            .setColor("0000ff")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")

        message.channel.send({embeds: [embed]})
    },
    requiredRoles: "Security",
}