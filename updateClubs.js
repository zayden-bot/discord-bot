const Discord = require("discord.js");

module.exports = async (client, channelId) => {
    const channel = await client.channels.fetch(channelId)

    const updatedClubs = new Discord.MessageEmbed()
        .setTitle(`Join a CK Custom Club!`)
        .setDescription(`
        :one: <@&805766527889440778>
        Wtf is that?

        :two: <@&805766587704803339>
        We're the gals that cheer on Steve! Join us to make Steve happy and GO STEVE! (only girls allowed btw)

        :three: <@&805766682202079273>
        We move the masses to start a revolution! Join now and come to the dark good side!

        :four: <@&805766763215847454>
        The only thing better than Chloe is Chloe with a dick! Join us to make Steve blush!

        :five: <@&805766896233742346>
        You like Lauren? We Lustin 4 Lauren. Join us to level up in your relationship with Lauren!

        :six: <@&809240184201412619>


        :seven: <@&809240269622214676>
        `)
        .setColor("0000ff")
        .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")

    channel.messages.fetch("805767878212452372").then((message) => { message.edit(updatedClubs) })
    // Club's Message ID ^
}