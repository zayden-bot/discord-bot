const Discord = require("discord.js");

module.exports = { 
    customClubs: async (client, channelId) => {
        const channel = await client.channels.fetch(channelId)

        const updatedClubs = new Discord.MessageEmbed()
            .setTitle(`Join a CK Custom Club!`)
            .setDescription(`
            :one: <@&805766527889440778>
            Come join our community events. You will be pinged when events are taking place!

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

            :eight: <@&825746784688668713>
            Those that don't want to be under the rule of a passive aggressive old man that want the right of freedom of speech.
            `)
            .setColor("0000ff")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")

        channel.messages.fetch("805767878212452372").then((message) => { message.edit(updatedClubs) })
        // Club's Message ID ^
    },
    pledgeRoles: async (client, channelId) => {
        const channel = await client.channels.fetch(channelId)

        const updatedRoles = new Discord.MessageEmbed()
            .setTitle("Who do you pledge for?")
            .setDescription(`
            :one: <@&818365011554336809>
            
            :two: <@&818365035259887626>
            `)
            .setColor("0000ff")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")

        channel.messages.fetch("818369390067449859").then((message) => { message.edit(updatedRoles) })
    }
}