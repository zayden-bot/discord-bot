const Discord = require("discord.js");

module.exports = { 
    customClubs: async (client, channelId) => {
        const channel = await client.channels.fetch(channelId)

        const embed = new Discord.MessageEmbed()
            .setTitle(`Join a CK Custom Club!`)
            .setDescription(`
            :one: <@&805766527889440778>
            Come join our community events. You will be pinged when events are taking place!

            :two: <@&805766587704803339>
            We're the gals that cheer on the Team! Join us to make the Team happy and GO STEVE!

            :three: <@&805766682202079273>
            We move the masses to start a revolution! Join now and come to the dark good side!

            :four: <@&805766763215847454>
            The only thing better than Chloe is Chloe with a dick! Join us to make Steve blush!

            :five: <@&805766896233742346>
            You like Lauren? We Lustin 4 Lauren. Join us to level up in your relationship with Lauren!

            :six: <@&809240184201412619>
            Do you like the version of Chloe without a dick? Cause over here we're crazy for her!

            :seven: <@&809240269622214676>
            Are you prepared to back up your hacking Queen? Penelope's Posse is right for you!

            :eight: <@&880607567074721822>
            Want to help keep the peace against the evil bulldog named OscarSix? Join the Peacekeepers and help Peace fight the so called revolution

            :nine: <@&836659038666227722>
            Ever wanted to finger the ginger? Now its your time to shine and show off your obsession with Riley
            `)
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")

        channel.messages.fetch("805767878212452372").then((message) => { message.edit({embeds: [embed]}) })
        // Club's Message ID ^
    },
    pledgeRoles: async (client, channelId) => {
        const channel = await client.channels.fetch(channelId)

        const embed = new Discord.MessageEmbed()
            .setTitle("Who do you pledge for?")
            .setDescription(`
            :one: <@&818365011554336809>
            
            :two: <@&818365035259887626>

            :three: <@&862417421167886357>
            `)
            .setColor("0000ff")
            .setThumbnail("https://images-ext-2.discordapp.net/external/QOCCliX2PNqo717REOwxtbvIrxVV2DZ1CRc8Svz3vUs/https/collegekingsgame.com/wp-content/uploads/2020/08/college-kings-wide-white.png?width=1440&height=566")

        channel.messages.fetch("818369390067449859").then((message) => { message.edit({embeds: [embed]}) })
    }
}
